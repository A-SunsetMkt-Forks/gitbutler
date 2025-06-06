<script lang="ts">
	import { goto } from '$app/navigation';
	import InfoMessage, { type MessageStyle } from '$components/InfoMessage.svelte';
	import Section from '$components/Section.svelte';
	import { PostHogWrapper } from '$lib/analytics/posthog';
	import { invoke } from '$lib/backend/ipc';
	import { ProjectsService } from '$lib/project/projectsService';
	import { parseRemoteUrl } from '$lib/url/gitUrl';
	import { getContext } from '@gitbutler/shared/context';
	import { persisted } from '@gitbutler/shared/persisted';
	import Button from '@gitbutler/ui/Button.svelte';
	import Spacer from '@gitbutler/ui/Spacer.svelte';
	import Textbox from '@gitbutler/ui/Textbox.svelte';
	import * as Sentry from '@sentry/sveltekit';
	import { documentDir } from '@tauri-apps/api/path';
	import { join } from '@tauri-apps/api/path';
	import { open } from '@tauri-apps/plugin-dialog';
	import { onMount } from 'svelte';

	const projectsService = getContext(ProjectsService);
	const posthog = getContext(PostHogWrapper);

	let loading = $state(false);
	let errors = $state<{ label: string }[]>([]);
	let completed = $state(false);
	let repositoryUrl = $state('');
	let targetDirPath = $state('');
	let savedTargetDirPath = persisted('', 'clone_targetDirPath');

	onMount(async () => {
		if ($savedTargetDirPath) {
			targetDirPath = $savedTargetDirPath;
		} else {
			targetDirPath = await documentDir();
		}
	});

	async function handleCloneTargetSelect() {
		const selectedPath = await open({
			directory: true,
			recursive: true,
			title: 'Target Clone Directory'
		});
		if (!selectedPath || !selectedPath[0]) return;

		targetDirPath = Array.isArray(selectedPath) ? selectedPath[0] : selectedPath;
	}

	async function cloneRepository() {
		loading = true;
		savedTargetDirPath.set(targetDirPath);
		if (errors.length) {
			errors = [];
		}

		if (!repositoryUrl || !targetDirPath) {
			errors.push({
				label: 'You must add both a repository URL and target directory.'
			});
			loading = false;
			return;
		}

		try {
			const remoteUrl = parseRemoteUrl(repositoryUrl);
			if (!remoteUrl) {
				return;
			}

			const targetDir = await join(targetDirPath, remoteUrl.name);

			await invoke('git_clone_repository', {
				repositoryUrl,
				targetDir
			});

			posthog.capture('Repository Cloned', { protocol: remoteUrl.protocol });
			await projectsService.addProject(targetDir);
		} catch (e) {
			Sentry.captureException(e);
			posthog.capture('Repository Clone Failure', { error: String(e) });
			errors.push({
				label: String(e)
			});
		} finally {
			loading = false;
		}
	}

	function handleCancel() {
		if (history.length > 0) {
			history.back();
		} else {
			goto('/');
		}
	}
</script>

<h1 class="clone-title text-serif-40">Clone a repository</h1>
<Section>
	<div class="clone__field repositoryUrl">
		<div class="text-13 text-semibold clone__field--label">Clone URL</div>
		<Textbox bind:value={repositoryUrl} />
	</div>
	<div class="clone__field repositoryTargetPath">
		<div class="text-13 text-semibold clone__field--label">Where to clone</div>
		<Textbox bind:value={targetDirPath} placeholder="/Users/tipsy/Documents" />
		<Button kind="outline" disabled={loading} onclick={handleCloneTargetSelect}>Choose..</Button>
	</div>
</Section>

<Spacer dotted margin={24} />

{#if completed}
	{@render Notification({ title: 'Success', style: 'success' })}
{/if}
{#if errors.length}
	{@render Notification({ title: 'Error', items: errors, style: 'error' })}
{/if}

<div class="clone__actions">
	<Button kind="outline" disabled={loading} onclick={handleCancel}>Cancel</Button>
	<Button
		style="pop"
		icon={errors.length > 0 ? 'update' : 'chevron-right-small'}
		disabled={loading}
		{loading}
		onclick={cloneRepository}
	>
		{#if loading}
			Cloning..
		{:else if errors.length > 0}
			Retry clone
		{:else}
			Clone
		{/if}
	</Button>
</div>

{#snippet Notification({
	title: titleLabel,
	items,
	style
}: {
	title: string;
	items?: any[];
	style: MessageStyle;
})}
	<div class="clone__info-message">
		<InfoMessage {style} filled outlined={false}>
			{#snippet title()}
				{titleLabel}
			{/snippet}
			{#snippet content()}
				{#if items && items.length > 0}
					{#each items as item}
						{@html item.label}
					{/each}
				{/if}
			{/snippet}
		</InfoMessage>
	</div>
{/snippet}

<style>
	.clone-title {
		margin-bottom: 20px;
		color: var(--clr-scale-ntrl-0);
		line-height: 1;
	}

	.clone__field {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.clone__field--label {
		color: var(--clr-scale-ntrl-50);
	}

	.clone__actions {
		display: flex;
		justify-content: end;
		gap: 8px;
	}

	.clone__info-message {
		margin-bottom: 20px;
	}
</style>
