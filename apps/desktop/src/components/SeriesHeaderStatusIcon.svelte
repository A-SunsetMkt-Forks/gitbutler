<script lang="ts">
	import Icon from '@gitbutler/ui/Icon.svelte';

	const FALLBACK_COLOR = 'var(--clr-scale-ntrl-80)';

	interface Props {
		icon: 'plus-small' | 'tick-small' | 'branch-small';
		iconColor?: string;
		color?: string;
		lineTop?: boolean;
		lineBottom?: boolean;
	}

	const {
		icon,
		iconColor,
		color = FALLBACK_COLOR,
		lineTop = true,
		lineBottom = true
	}: Props = $props();
</script>

<div class="stack__status gap">
	<div
		class="stack__status--bar"
		style:--bg-color={lineTop ? color : 'var(--clr-transparent)'}
	></div>
	<div class="stack__status--icon" style:--bg-color={color} style:--icon-color={iconColor}>
		<Icon name={icon} />
	</div>
	<div
		class="stack__status--bar last"
		style:--bg-color={lineBottom ? color : 'var(--clr-transparent)'}
	></div>
</div>

<style>
	.stack__status {
		display: flex;
		flex-direction: column;
		align-self: stretch;
		justify-content: center;
		gap: 3px;
		--clr-transparent: transparent;

		& .stack__status--icon {
			display: flex;
			align-items: center;
			justify-content: center;
			width: 20px;
			height: 24px;
			margin-left: 11px;
			border-radius: var(--radius-m);
			background-color: var(--bg-color);
			color: var(--icon-color, var(--clr-text-1));
		}

		& .stack__status--bar {
			width: 2px;
			height: 10px;
			margin: 0 22px 0 20px;
			background: var(--bg-color);

			&.last {
				flex: 1;
			}
		}
	}
</style>
