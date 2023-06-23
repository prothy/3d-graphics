<script lang="ts">
    import { onMount } from 'svelte';
    import { initializeWebGlContext } from '../../utils';

    onMount(() => {
        const gl = initializeWebGlContext();

        if (!gl) {
            return;
        }
    });
</script>

<h1>Square</h1>

<canvas />
