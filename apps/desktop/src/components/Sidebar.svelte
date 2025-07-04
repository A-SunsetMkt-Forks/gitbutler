<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import SupportersBanner from '$components/SupportersBanner.svelte';
	import { platformName } from '$lib/platform/platform';
	import { User } from '$lib/user/user';
	import { openExternalUrl } from '$lib/utils/url';
	import { getContextStore } from '@gitbutler/shared/context';
	import Button from '@gitbutler/ui/Button.svelte';
	import Icon from '@gitbutler/ui/Icon.svelte';

	const currentSection: string | undefined = $derived(getPageName($page.url.pathname));

	const settingsPageRegExp = /\/settings\/(.*?)(?:$|\/)/;
	const user = getContextStore(User);

	function getPageName(pathname: string) {
		const matches = pathname.match(settingsPageRegExp);

		return matches?.[1];
	}

	function onMenuClick(section: string) {
		goto(`/settings/${section}`, { replaceState: true });
	}
</script>

<aside class="profile-sidebar">
	<section class="profile-sidebar__top">
		<div class="profile-sidebar__menu-wrapper">
			<div class="profile-sidebar__header-wrapper">
				{#if platformName === 'macos'}
					<div class="traffic-lights-placeholder"></div>
				{/if}
				<div class="profile-sidebar__header">
					<div class="back-btn__icon">
						<Button
							icon="chevron-left"
							kind="ghost"
							onmousedown={() => {
								if (history.length > 0) {
									history.back();
								} else {
									goto('/');
								}
							}}
						/>
					</div>
					<h2 class="profile-sidebar__title text-18 text-bold">Preferences</h2>
				</div>
			</div>

			<ul class="profile-sidebar__menu">
				<li>
					<button
						type="button"
						class="profile-sidebar__menu-item"
						class:item_selected={currentSection === 'general'}
						onmousedown={() => onMenuClick('general')}
					>
						<Icon name="settings" />
						<span class="text-14 text-semibold">General</span>
					</button>
				</li>
				<li>
					<button
						type="button"
						class="profile-sidebar__menu-item"
						class:item_selected={currentSection === 'appearance'}
						onmousedown={() => onMenuClick('appearance')}
					>
						<Icon name="appearance" />
						<span class="text-14 text-semibold">Appearance</span>
					</button>
				</li>
				<li>
					<button
						type="button"
						class="profile-sidebar__menu-item"
						class:item_selected={currentSection === 'git'}
						onmousedown={() => onMenuClick('git')}
					>
						<Icon name="git" />
						<span class="text-14 text-semibold">Git stuff</span>
					</button>
				</li>

				<li>
					<button
						type="button"
						class="profile-sidebar__menu-item"
						class:item_selected={currentSection === 'integrations'}
						onmousedown={() => onMenuClick('integrations')}
					>
						<Icon name="integrations" />
						<span class="text-14 text-semibold">Integrations</span>
					</button>
				</li>
				<li>
					<button
						type="button"
						class="profile-sidebar__menu-item"
						class:item_selected={currentSection === 'ai'}
						onmousedown={() => onMenuClick('ai')}
					>
						<Icon name="ai" />
						<span class="text-14 text-semibold">AI options</span>
					</button>
				</li>
				<li>
					<button
						type="button"
						class="profile-sidebar__menu-item"
						class:item_selected={currentSection === 'telemetry'}
						onmousedown={() => onMenuClick('telemetry')}
					>
						<Icon name="stat" />
						<span class="text-14 text-semibold">Telemetry</span>
					</button>
				</li>
				<li>
					<button
						type="button"
						class="profile-sidebar__menu-item"
						class:item_selected={currentSection === 'experimental'}
						onmousedown={() => onMenuClick('experimental')}
					>
						<Icon name="idea" />
						<span class="text-14 text-semibold">Experimental</span>
					</button>
				</li>
				{#if $user?.role === 'admin'}
					<li>
						<button
							type="button"
							class="profile-sidebar__menu-item"
							class:item_selected={currentSection === 'organizations'}
							onmousedown={() => onMenuClick('organizations')}
						>
							<Icon name="idea" />
							<span class="text-14 text-semibold">Organizations</span>
						</button>
					</li>
				{/if}
			</ul>
		</div>
	</section>

	<section class="profile-sidebar__bottom">
		<SupportersBanner />

		<div class="social-banners">
			<button
				type="button"
				class="social-banner"
				onclick={async () =>
					await openExternalUrl('mailto:hello@gitbutler.com?subject=Feedback or question!')}
			>
				<span class="text-14 text-bold">Contact us</span>
				<Icon name="mail" />
			</button>
			<button
				type="button"
				class="social-banner"
				onclick={async () => await openExternalUrl('https://discord.gg/MmFkmaJ42D')}
			>
				<span class="text-14 text-bold">Join our Discord</span>
				<Icon name="discord" />
			</button>
		</div>
	</section>
</aside>

<style lang="postcss">
	.profile-sidebar {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		width: 256px;
		height: 100%;
		padding: 14px;
		border-right: 1px solid var(--clr-border-2);
		background-color: var(--clr-bg-1);
	}

	.profile-sidebar__header-wrapper {
		display: flex;
		flex-direction: column;
	}

	.profile-sidebar__header {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.traffic-lights-placeholder {
		width: 100%;
		height: 24px;
	}

	/* TOP */

	.profile-sidebar__top {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.profile-sidebar__title {
		color: var(--clr-scale-ntrl-0);
	}

	/* MENU */

	.profile-sidebar__menu-wrapper {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.profile-sidebar__menu {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.profile-sidebar__menu-item {
		display: flex;
		align-items: center;
		width: 100%;
		padding: 10px 8px;
		gap: 10px;
		border-radius: var(--radius-m);
		color: var(--clr-scale-ntrl-30);
		transition:
			background-color var(--transition-fast),
			color var(--transition-fast);

		&:not(.item_selected):hover {
			background-color: var(--clr-bg-1-muted);
			transition: none;
		}

		& span {
			color: var(--clr-scale-ntrl-0);
		}
	}

	.item_selected {
		background-color: var(--clr-bg-2);
		color: var(--clr-scale-ntrl-0);
	}

	/* BOTTOM */
	.profile-sidebar__bottom {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	/* BANNERS */
	.social-banners {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.social-banner {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px;
		border: 1px solid var(--clr-border-2);
		border-radius: var(--radius-m);
		background-color: var(--clr-bg-1);
		color: var(--clr-scale-ntrl-30);
		transition: background-color var(--transition-fast);

		&:hover {
			background-color: var(--clr-bg-1-muted);
		}
	}
</style>
