<script lang="ts">
	import * as jsonLinks from '$home/data/links.json';
	import { onMount } from 'svelte';

	let videoElement = $state<HTMLVideoElement>();
	let io: IntersectionObserver;

	onMount(() => {
		const ioOptions = {
			root: null,
			rootMargin: '0% 0% -20% 0%',
			threshold: 0
		};

		io = new IntersectionObserver((entries) => {
			entries.forEach((entry) => {
				if (entry.isIntersecting) {
					videoElement?.play();
				} else {
					videoElement?.pause();
				}
			});
		}, ioOptions);

		io.observe(videoElement as Element);

		return () => {
			io.disconnect();
		};
	});
</script>

<a href={jsonLinks.other['youtube-demo'].url} target="_blank" class="yt-preview">
	<img class="yt-preview__btn" src="/images/video-thumb/watch-btn.svg" alt="" />

	<div class="yt-preview__overlay-patttern"></div>

	<video
		bind:this={videoElement}
		class="yt-preview__video"
		loop
		muted
		playsinline
		preload="auto"
		src="/images/video-thumb/video-thumb-loop.mp4#t=0.1"
	></video>
</a>

<style lang="scss">
	.yt-preview {
		display: flex;
		position: relative;
		align-items: center;
		justify-content: center;
		aspect-ratio: 16/9;
		width: 100%;
		height: 100%;
		overflow: hidden;
		border-radius: 10px;
		background-color: var(--clr-black);
		transition: transform 0.2s ease-in-out;

		&__btn {
			z-index: 2;
			position: absolute;
			top: 50%;
			left: 50%;
			height: 40px;
			transform: translate(-50%, -50%);
		}

		&__overlay-patttern {
			z-index: 1;
			position: absolute;
			top: 0;
			left: 0;
			width: 100%;
			height: 100%;
			background: url('/images/video-thumb/overlay-pattern.svg') repeat;
			background-size: 4px;
			opacity: 0.3;
		}

		&__video {
			z-index: 0;
			width: calc(100% + 4px);
			object-fit: cover;
		}

		@media (min-width: 500px) {
			display: none;
		}
	}
</style>
