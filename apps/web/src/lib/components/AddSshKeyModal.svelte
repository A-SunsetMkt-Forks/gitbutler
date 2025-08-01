<script lang="ts">
	import { SSH_KEY_SERVICE } from '$lib/sshKeyService';
	import { inject } from '@gitbutler/shared/context';
	import { AsyncButton, Button, Modal, Textarea, Textbox } from '@gitbutler/ui';

	const sshKeyService = inject(SSH_KEY_SERVICE);
	let name = $state('');
	let publicKey = $state('');
	let error = $state<string | null>(null);

	const { onClose } = $props<{
		onClose: () => void;
	}>();

	async function handleSubmit() {
		if (!name.trim() || !publicKey.trim()) {
			error = 'Please fill in all fields';
			return;
		}

		error = null;

		try {
			await sshKeyService.addSshKey(name.trim(), publicKey.trim());
			// Close modal and reset form
			name = '';
			publicKey = '';
			modal?.close();
			onClose();
		} catch (err) {
			console.error('Failed to add SSH key:', err);
			error = 'Failed to add SSH key. Please try again.';
		}
	}

	let modal = $state<Modal>();

	export function show() {
		modal?.show();
	}
</script>

<Modal bind:this={modal} {onClose} title="Add SSH Key">
	<div class="container">
		<p class="description">
			Add a new SSH key to your account. You can find your public key in your SSH key file (usually
			ending in .pub).
		</p>

		<Textbox label="Key name" placeholder="e.g., MacBook Pro" bind:value={name} required={false} />

		<Textarea
			label="Public Key"
			placeholder="ssh-rsa AAAAB3NzaC1yc2EAAAADA..."
			bind:value={publicKey}
			minRows={6}
			required={false}
		/>

		{#if error}
			<div class="error-key">{error}</div>
		{/if}
	</div>

	{#snippet controls()}
		<Button onclick={() => modal?.close()} kind="outline">Cancel</Button>
		<AsyncButton action={handleSubmit} style="pop">Add Key</AsyncButton>
	{/snippet}
</Modal>

<style lang="postcss">
	.error-key {
		color: red;
	}

	.container {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.description {
		color: var(--clr-scale-ntrl-30);
		font-size: 14px;
		line-height: 1.5;
	}
</style>
