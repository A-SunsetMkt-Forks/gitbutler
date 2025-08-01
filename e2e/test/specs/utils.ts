import getPort from 'get-port';
import { dir } from 'tmp-promise';
import { spawn } from 'node:child_process';
import { Socket } from 'node:net';
import * as path from 'node:path';

interface GitButler {
	visit(path: string): Promise<void>;
	cleanup(): Promise<void>;
}

const VITE_HOST = 'localhost';
const BUTLER_HOST = 'localhost';

// Colors for console output
const colors = {
	reset: '\x1b[0m',
	bright: '\x1b[1m',
	green: '\x1b[32m',
	yellow: '\x1b[33m',
	red: '\x1b[31m',
	blue: '\x1b[34m',
	cyan: '\x1b[36m'
};

function log(message: string, color = colors.reset) {
	// eslint-disable-next-line no-console
	console.log(`${color}${message}${colors.reset}`);
}

function spawnProcess(
	command: string,
	args: string[],
	cwd = process.cwd(),
	env: Record<string, string> = {}
) {
	return spawn(command, args, {
		cwd,
		stdio: 'inherit',
		env: {
			...process.env,
			ELECTRON_ENV: 'development',
			VITE_BUILD_TARGET: 'web',
			VITE_HOST,
			BUTLER_HOST: '0.0.0.0',
			...env
		}
	});
}

async function runCommand(command: string, args: string[], cwd = process.cwd()) {
	return await new Promise<void>((resolve, reject) => {
		log(`Running: ${command} ${args.join(' ')}`, colors.cyan);

		const child = spawnProcess(command, args, cwd);

		child.on('close', (code) => {
			if (code === 0) {
				resolve();
			} else {
				reject(new Error(`Command failed with exit code ${code}`));
			}
		});

		child.on('error', (error) => {
			reject(error);
		});
	});
}

async function checkPort(port: number, host = 'localhost') {
	return await new Promise((resolve) => {
		const socket = new Socket();

		socket.setTimeout(500);
		socket.on('connect', () => {
			socket.destroy();
			resolve(true);
		});

		socket.on('timeout', () => {
			socket.destroy();
			resolve(false);
		});

		socket.on('error', () => {
			resolve(false);
		});

		socket.connect(port, host);
	});
}

async function waitForServer(port: number, host = 'localhost', maxAttempts = 500) {
	// log(`Waiting for server on ${host}:${port}...`, colors.yellow);

	for (let i = 0; i < maxAttempts; i++) {
		if (await checkPort(port, host)) {
			// log(`✅ Server is ready on ${host}:${port}`, colors.green);
			return true;
		}

		if (i < maxAttempts - 1) {
			await new Promise((resolve) => setTimeout(resolve, 1000));
		}
	}

	return false;
}

let builtDesktop = false;

export async function startGitButler(browser: WebdriverIO.Browser): Promise<GitButler> {
	const configDir = await dir({ unsafeCleanup: true });
	const workDir = await dir({ unsafeCleanup: true });

	const vitePort = await getPort();
	const butPort = await getPort();

	// Get paths
	const rootDir = path.resolve(import.meta.dirname, '../../..');
	const desktopDir = path.resolve(rootDir, 'apps/desktop');

	// Start the Vite dev server
	if (!builtDesktop) {
		await runCommand('pnpm', ['build:desktop'], rootDir);
		builtDesktop = true;
	}
	const viteProcess = spawnProcess('pnpm', ['preview', '--port', `${vitePort}`], desktopDir);

	viteProcess.on('close', (code) => {
		if (code !== 0 && code !== null) {
			log(`Vite dev server exited with code ${code}`, colors.red);
		}
	});

	viteProcess.on('error', (error) => {
		log(`Vite dev server error: ${error.message}`, colors.red);
	});

	// Start the but-server server
	const butProcess = spawnProcess('cargo', ['run', '-p', 'but-server'], rootDir, {
		VITE_PORT: `${vitePort}`,
		BUTLER_PORT: `${butPort}`,
		E2E_TEST_APP_DATA_DIR: configDir.path
	});

	butProcess.on('close', (code) => {
		if (code !== 0 && code !== null) {
			log(`Butler server exited with code ${code}`, colors.red);
		}
	});

	butProcess.on('error', (error) => {
		log(`Butler server error: ${error.message}`, colors.red);
	});

	// Wait for Vite to be ready
	const butReady = await waitForServer(butPort, BUTLER_HOST);
	// Wait for Vite to be ready
	const serverReady = await waitForServer(vitePort, VITE_HOST);

	if (!butReady) {
		throw new Error(`Butler server failed to start on ${BUTLER_HOST}:${butPort}`);
	}
	if (!serverReady) {
		throw new Error(`Vite dev server failed to start on ${VITE_HOST}:${vitePort}`);
	}

	return {
		async visit(path: string) {
			if (path.startsWith('/')) {
				path = path.slice(1);
			}

			await browser.url(`http://${VITE_HOST}:${vitePort}/${path}`);
			await browser.setCookies([
				{
					name: 'butlerPort',
					value: `${butPort}`
				},
				{
					name: 'butlerHost',
					value: BUTLER_HOST
				}
			]);
		},
		async cleanup() {
			// log('Stopping Vite dev server...', colors.yellow);
			viteProcess.kill();
			// log('Stopping butler server...', colors.yellow);
			butProcess.kill();
			await configDir.cleanup();
			await workDir.cleanup();
		}
	};
}
