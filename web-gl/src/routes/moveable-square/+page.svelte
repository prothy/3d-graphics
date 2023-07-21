<script lang="ts">
    import { onMount } from 'svelte';
    import GL from './gl';

    onMount(() => {
        const gl = new GL();

        gl.handleBuffer([10, 20, 800, 20, 10, 300, 10, 300, 800, 20, 800, 300]);
        gl.draw();
    });
</script>

<h1>Hello World</h1>

<canvas />
