<script lang="ts">
    import { onMount } from 'svelte';
    import { createProgram, createShader, initializeWebGlContext } from '../../utils';

    import vsSource from './shader.vert?raw';
    import fsSource from './shader.frag?raw';

    //////////////////////////////////////////////////////////////////////////////////
    //                                                                              //
    //    https://webgl2fundamentals.org/webgl/lessons/webgl-2d-translation.html    //
    //                                                                              //
    //////////////////////////////////////////////////////////////////////////////////

    function createShaders(gl: WebGL2RenderingContext) {
        const vertexShader = createShader(gl, gl.VERTEX_SHADER, vsSource);
        const fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fsSource);

        if (!vertexShader || !fragmentShader) {
            return;
        }

        return [vertexShader, fragmentShader];
    }

    

    onMount(() => {
        const gl = initializeWebGlContext();

        if (!gl) {
            return;
        }

        const shaders = createShaders(gl);

        if (!shaders) {
            return;
        }

        const program = createProgram(gl, shaders);

        if (!program) {
            return;
        }

        const positionAttributeLocation = gl.getAttribLocation(program, 'a_position');
        const resolutionUniformLocation = gl.getUniformLocation(program, 'u_resolution');
        const translationLocation = gl.getUniformLocation(program, 'u_translation')

        const positionBuffer = gl.createBuffer();

        gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

        // prettier-ignore
        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([
            150, 200, 800, 
            200, 150, 700, 
            150, 700, 800, 
            200, 800, 700
        ]), gl.STATIC_DRAW);

        const vao = gl.createVertexArray();
        gl.bindVertexArray(vao);

        gl.enableVertexAttribArray(positionAttributeLocation);

        const size = 2;
        const type = gl.FLOAT;
        const normalize = false;
        const stride = 0;
        const offset = 0;

        gl.vertexAttribPointer(positionAttributeLocation, size, type, normalize, stride, offset);

        const canvas = document.querySelector('canvas');

        if (canvas) {
            canvas.width = canvas?.clientWidth;
            canvas.height = canvas?.clientHeight;
        }

        gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);

        let translationMultiplier = 0;

        // while(true) {
            {
            translationMultiplier += 1;

            gl.clearColor(0, 0, 0, 0);
            gl.clear(gl.COLOR_BUFFER_BIT);

            gl.useProgram(program);

            gl.uniform2f(resolutionUniformLocation, gl.canvas.width, gl.canvas.height);
            gl.uniform2f(translationLocation, -100, 0)

            gl.bindVertexArray(vao);

            gl.drawArrays(gl.TRIANGLES, 0, 6);}
        // }
    });
</script>

<h1>Hello World</h1>

<canvas />
