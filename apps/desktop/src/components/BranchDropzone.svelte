<script lang="ts">
	// import images
	import Dropzone from '$components/Dropzone.svelte';
	import bottomSheetSvg from '$lib/assets/new-branch/bottom-sheet.svg?raw';
	import handSvg from '$lib/assets/new-branch/hand.svg?raw';
	import middleSheetSvg from '$lib/assets/new-branch/middle-sheet.svg?raw';
	import topSheetSvg from '$lib/assets/new-branch/top-sheet.svg?raw';
	// import components
	import { NewStackDzHandler } from '$lib/stacks/dropHandler';
	import { StackService } from '$lib/stacks/stackService.svelte';
	import { getContext } from '@gitbutler/shared/context';
	import Button from '@gitbutler/ui/Button.svelte';

	type Props = {
		projectId: string;
	};

	const { projectId }: Props = $props();

	const stackService = getContext(StackService);
	const handler = $derived(new NewStackDzHandler(stackService, projectId));
</script>

<div class="canvas-dropzone">
	<Dropzone handlers={[handler]}>
		{#snippet overlay({ hovered, activated })}
			<div class="new-virtual-branch" class:activated class:hovered>
				<div class="new-virtual-branch__content">
					<div class="stimg">
						<div class="stimg__hand">
							{@html handSvg}
						</div>
						<div class="stimg__top-sheet">
							{@html topSheetSvg}
						</div>
						<div class="stimg__middle-sheet">
							{@html middleSheetSvg}
						</div>
						<div class="stimg__bottom-sheet">
							{@html bottomSheetSvg}
						</div>

						<div class="stimg__branch">
							<div class="stimg__branch-plus"></div>
						</div>
					</div>

					<span class="text-13 text-body new-branch-caption">
						Drag and drop files<br />to create a new branch
					</span>
				</div>
				<div class="new-branch-button">
					<Button
						kind="outline"
						icon="plus-small"
						onmousedown={async () => await stackService.newStackMutation({ projectId, branch: {} })}
						>New branch</Button
					>
				</div>
			</div>
		{/snippet}
	</Dropzone>
</div>

<style lang="postcss">
	.canvas-dropzone {
		display: flex;
		height: 100%;
		padding: 12px;
		user-select: none;
	}

	.new-virtual-branch {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		width: 352px;
		height: 100%;
		padding: 20px;
		gap: 8px;
		border: 1px dashed var(--clr-border-2);
		border-radius: var(--radius-m);

		outline-color: transparent;
		outline-style: dashed;
		outline-width: 1px;
		outline-offset: -1px;
		background-color: transparent;

		transition:
			outline-offset var(--transition-medium),
			outline-color var(--transition-medium),
			border var(--transition-medium),
			background-color var(--transition-medium);
	}

	.new-virtual-branch__content {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 20px 24px 16px 24px;
		gap: 12px;
		transition: transform var(--transition-medium);
	}

	/* ILLUSTRATION */
	.stimg {
		position: relative;
		width: 120px;
		height: 130px;
		opacity: 0.8;

		& div {
			position: absolute;
			transition:
				transform var(--transition-medium),
				opacity var(--transition-medium),
				filter var(--transition-medium);
		}
	}

	.stimg__hand {
		z-index: 5;
		top: 85px;
		left: 25px;
	}

	.stimg__top-sheet {
		z-index: 4;
		top: 56px;
		left: 8px;
		width: 44px;
		height: 53px;
	}

	.stimg__middle-sheet {
		z-index: 3;
		top: 42px;
		left: 23px;
		width: 44px;
		height: 40px;
	}

	.stimg__bottom-sheet {
		z-index: 2;
		top: 66px;
		left: 44px;
		width: 44px;
		height: 41px;
	}

	.stimg__branch {
		z-index: 1;
		top: 0;
		left: 35px;
		width: 77px;
		height: 83px;
		border-radius: 12px;
		background-color: oklch(from var(--clr-scale-ntrl-60) l c h / 0.2);
	}

	.stimg__branch-plus {
		position: absolute;
		top: 16px;
		left: 50%;
		width: 34px;
		height: 34px;
		transform: translateX(-50%);
		opacity: 0.3;

		&::before,
		&::after {
			position: absolute;
			top: 50%;
			left: 50%;
			width: 100%;
			height: 2px;
			transform: translate(-50%, -50%);
			background-color: var(--clr-scale-ntrl-20);
			content: '';
		}

		&::after {
			transform: translate(-50%, -50%) rotate(90deg);
		}
	}

	/* OTHER */

	.new-branch-caption {
		color: var(--clr-scale-ntrl-0);
		text-align: center;
		opacity: 0.25;
		transition: opacity var(--transition-medium);
	}

	.new-branch-button {
		transition: opacity var(--transition-medium);
	}

	/* DRAGZONE MODIEFIERS */
	.activated {
		&.new-virtual-branch {
			border: 1px dashed oklch(from var(--clr-scale-pop-40) l c h / 0.8);
			background-color: oklch(from var(--clr-scale-pop-70) l c h / 0.1);
			color: var(--clr-scale-pop-50);
		}

		& .new-virtual-branch__content {
			transform: translateY(0.5rem);
		}

		& .new-branch-button {
			opacity: 0;
		}

		& .stimg {
			opacity: 1;
		}

		& .stimg__hand {
			transform: translate(4px, 3px) rotate(5deg) scale(0.9);
		}

		& .stimg__top-sheet {
			transform: translate(3px, 4px);
		}

		& .stimg__middle-sheet {
			transform: translate(-2px, 7px);
		}

		& .stimg__bottom-sheet {
			transform: translate(-5px, 0);
		}

		& .stimg__branch-plus {
			opacity: 0.4;

			&::before,
			&::after {
				width: 110%;
			}
		}
	}
</style>
