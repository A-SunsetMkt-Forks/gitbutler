<script lang="ts">
	import { AuthService } from '$lib/auth/authService.svelte';
	import { UserService } from '$lib/user/userService';
	import { getContext } from '@gitbutler/shared/context';

	const authService = getContext(AuthService);
	const userService = getContext(UserService);

	const user = $derived(userService.user);
	const token = $derived(authService.tokenReadable);
	let userAvatarUrl = $state($user?.picture);

	function handleImageLoadError() {
		userAvatarUrl = `https://unavatar.io/${$user?.email}`;
	}
</script>

{#if !$token}
	<p>Unauthorized</p>
{:else if !$user?.id}
	<p>Loading...</p>
{:else}
	<div class="user__wrapper">
		<div class="user__id">
			{#if $user?.picture}
				<img
					class="user__id--img"
					alt="User Avatar"
					width="48"
					src={userAvatarUrl}
					onerror={handleImageLoadError}
				/>
			{/if}
			<div class="user__id--name">{$user?.name}</div>
		</div>
		<div><b>Email</b>: {$user?.email}</div>
		<div><b>Joined</b>: {$user?.created_at}</div>
		<div><b>Supporter</b>: {$user?.supporter}</div>
	</div>
{/if}

<style>
	.user__wrapper {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
	.user__id {
		display: flex;
		align-items: center;
		gap: 0.5rem;

		.user__id--img {
			border-radius: 0.5rem;
		}
	}
</style>
