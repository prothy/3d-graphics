<script lang="ts">
    import { onMount } from 'svelte';
    import { initializeWebGlContext, loadShader } from '../../utils';
    import vsSource from './shader.vert?raw';
    import fsSource from './shader.frag?raw';

    onMount(() => {
        const gl = initializeWebGlContext();

        if (!gl) {
            return;
        }

        ////////////////////////////////////////////////////////////////////////////
        //                                                                        //
        //    FOLLOWING https://learnopengl.com/Getting-started/Hello-Triangle    //
        //                                                                        //
        ////////////////////////////////////////////////////////////////////////////

        const vertexShader = gl.createShader(gl.VERTEX_SHADER)!;
        gl.shaderSource(vertexShader, vsSource);
        gl.compileShader(vertexShader);

        console.log(gl.getShaderInfoLog(vertexShader));

        const fragmentShader = gl.createShader(gl.FRAGMENT_SHADER)!;
        gl.shaderSource(fragmentShader, fsSource);
        gl.compileShader(fragmentShader);

        console.log(gl.getShaderInfoLog(fragmentShader));

        const shaderProgram = gl.createProgram()!;

        gl.attachShader(shaderProgram, vertexShader);
        gl.attachShader(shaderProgram, fragmentShader);
        gl.linkProgram(shaderProgram);
        gl.useProgram(shaderProgram);

        // shaders no longer needed after linking to program
        gl.deleteShader(vertexShader);
        gl.deleteShader(fragmentShader);

        const vertices = new Float32Array([-0.5, -0.5, 0, 0.5, -0.5, 0, 0, 0.5, 0]);
        const colors = new Float32Array([
            1.0,
            0.0,
            0.0, // Color #1 (red)
            0.0,
            1.0,
            0.0, // Color #2 (green)
            0.0,
            0.0,
            1.0 // Color #3 (blue)
        ]);

        // VAO (vertex array object): stores state
        // when we want to draw an object, we bind VAO and attribute pointers (settings)
        const vao = gl.createVertexArray();

        // VBO (vertex buffer object): Store vertices in GPU
        const vbo = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, vbo); // vbo has GL_ARRAY_BUFFER type
        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW); // copy vertex data into buffer's memory
        // last parameter determines how GPU should manage data
        // STREAM_DRAW: data is set once and used seldomly by GPU
        // STATIC_DRAW: data is set once and used often by GPU
        // DYNAMIC_DRAW: data is changed and used often

        // stride can be set to 0 if tightly packed, webgl can determine it automatically
        gl.vertexAttribPointer(0, 3, gl.FLOAT, false, 0, 0);
        gl.enableVertexAttribArray(0);

        gl.bindVertexArray(vao);

        requestAnimationFrame(() => {
            gl.drawArrays(gl.TRIANGLES, 0, 3);
        });
    });
</script>

<h1>Square</h1>

<canvas />
