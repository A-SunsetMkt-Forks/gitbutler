<script lang="ts">
	import tryV3Svg from '$lib/assets/try-v3.svg?raw';
	import { SettingsService } from '$lib/config/appSettingsV2';
	import { getContext } from '@gitbutler/shared/context';
	import { persisted } from '@gitbutler/shared/persisted';
	import Button from '@gitbutler/ui/Button.svelte';
	import Checkbox from '@gitbutler/ui/Checkbox.svelte';
	import Modal from '@gitbutler/ui/Modal.svelte';
	import Link from '@gitbutler/ui/link/Link.svelte';
	import { onMount } from 'svelte';

	let modalRef = $state<ReturnType<typeof Modal>>();
	const doNotShowAgain = persisted<boolean>(false, 'doNotShowV3Modal');
	const settingsService = getContext(SettingsService);
	const settingsStore = settingsService.appSettings;
	const isV3Enabled = $derived($settingsStore?.featureFlags.v3);

	onMount(() => {
		if (!$doNotShowAgain && !isV3Enabled) {
			modalRef?.show();
		}
	});
</script>

<Modal
	width={434}
	bind:this={modalRef}
	noPadding
	onSubmit={async (close) => {
		settingsService.updateFeatureFlags({ v3: true });
		close();
	}}
>
	<div class="modal-wrapper">
		<div class="modal-illustration-wrapper">
			<div class="modal-illustration__svg">
				{@html tryV3Svg}
			</div>
		</div>
		<div class="modal-content">
			<div class="modal-description">
				<h2 class="text-16 text-bold">GitButler has a new, updated UI!</h2>
				<p class="text-13 text-body">
					In an upcoming release, this will be the default, so we'd love to know what you think!
				</p>
				<p class="text-13 text-body">
					Ping us on <Link href="https://discord.gg/MmFkmaJ42D">Discord</Link>, or create a <Link
						href="https://github.com/gitbutlerapp/gitbutler/issues/new?template=BLANK_ISSUE"
						>GitHub issue</Link
					>.
				</p>
			</div>

			<div class="modal-content__notes text-12 text-body">
				<p>This can also be toggled this under 'Experimental' in ⚙️ global settings.</p>

				<p class="clr-text-2">Known issues:</p>
				<ul class="clr-text-2">
					<li>- A restart may be needed for the change to fully take effect</li>
					<li>
						- It is currently not possible to assign uncommitted changes to a lane
						<Link href="https://github.com/gitbutlerapp/gitbutler/issues/8637">GitHub Issue</Link>
					</li>
				</ul>
			</div>
		</div>
	</div>
	{#snippet controls(close)}
		<div class="modal-footer">
			<label for="dont-show-again" class="modal-footer__checkbox">
				<Checkbox name="dont-show-again" small bind:checked={$doNotShowAgain} />
				<span class="text-12"> Don't ask again</span>
			</label>
			<Button kind="outline" testId="v3-not-now" onclick={close}>Not now</Button>
			<Button style="pop" type="submit">Switch to V3 UI</Button>
		</div>
	{/snippet}
</Modal>

<style lang="postcss">
	.modal-wrapper {
		display: flex;
		flex-direction: column;
	}

	.modal-illustration-wrapper {
		position: relative;
		height: 176px;
		background-color: var(--clr-illustration-bg);
	}

	.modal-illustration__svg {
		position: absolute;
		bottom: 0;
		left: 16px;
		width: 404px;
		height: 158px;
	}

	.modal-content {
		display: flex;
		flex-direction: column;
		padding: 20px 16px 16px;
		gap: 10px;
	}

	.modal-description {
		display: flex;
		flex-direction: column;
		max-width: 380px;
		gap: 10px;
	}

	.modal-content__notes {
		display: flex;
		flex-direction: column;
		margin-top: 8px;
		padding: 12px;
		gap: 2px;
		/* background-color: var(--clr-bg-1-muted); */
		border: 1px solid var(--clr-border-3);
		border-radius: var(--radius-m);
	}

	.modal-footer {
		display: flex;
		width: 100%;
		gap: 6px;
	}

	.modal-footer__checkbox {
		display: flex;
		flex: 1;
		align-items: center;
		gap: 8px;
	}
</style>
