<script lang="ts">
    import { onMount } from 'svelte';
    import { initializeWebGlContext } from '../../utils';

    import vsSource from './shader.vert?raw';
    import fsSource from './shader.frag?raw';

    ////////////////////////////////////////////////////////////////////////////////
    //                                                                            //
    //    https://webgl2fundamentals.org/webgl/lessons/webgl-fundamentals.html    //
    //                                                                            //
    ////////////////////////////////////////////////////////////////////////////////

    function createShader(gl: WebGL2RenderingContext, type: number, source: string) {
        const shader = gl.createShader(type);

        if (!shader) {
            return;
        }

        gl.shaderSource(shader, source);
        gl.compileShader(shader);

        const success = gl.getShaderParameter(shader, gl.COMPILE_STATUS);

        if (success) {
            return shader;
        }

        console.log(gl.getShaderInfoLog(shader));
        gl.deleteShader(shader);
    }

    function createProgram(
        gl: WebGL2RenderingContext,
        vertexShader: WebGLShader,
        fragmentShader: WebGLShader
    ) {
        const program = gl.createProgram();

        if (!program) {
            return;
        }

        gl.attachShader(program, vertexShader);
        gl.attachShader(program, fragmentShader);

        gl.linkProgram(program);

        const success = gl.getProgramParameter(program, gl.LINK_STATUS);

        if (success) {
            return program;
        }

        console.log(gl.getProgramInfoLog(program));
        gl.deleteShader(program);
    }

    onMount(() => {
        const gl = initializeWebGlContext();

        if (!gl) {
            return;
        }

        const vertexShader = createShader(gl, gl.VERTEX_SHADER, vsSource);
        const fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fsSource);

        if (!vertexShader || !fragmentShader) {
            return;
        }

        const program = createProgram(gl, vertexShader, fragmentShader);

        if (!program) {
            return;
        }

        const positionAttributeLocation = gl.getAttribLocation(program, 'a_position');

        const positionBuffer = gl.createBuffer();

        // u_resolution (see vertex shader) will take positions supplied in positionsBuffer supplied in pixels coordinates and convert to clip space
        const resolutionUniformLocation = gl.getUniformLocation(program, 'u_resolution');

        const colorLocation = gl.getUniformLocation(program, 'u_color');

        gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

        // new positions for rectangle
        const positions = [10, 20, 800, 20, 10, 300, 10, 300, 800, 20, 800, 300];

        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

        const vao = gl.createVertexArray();
        gl.bindVertexArray(vao);

        gl.enableVertexAttribArray(positionAttributeLocation);

        const size = 2; // 2 components per iteration
        const type = gl.FLOAT; // data is 32-bit floags
        const normalize = false;
        const stride = 0;
        const offset = 0;

        gl.vertexAttribPointer(positionAttributeLocation, size, type, normalize, stride, offset);

        const canvas = document.querySelector('canvas');

        if (canvas) {
            canvas.width = canvas?.clientWidth;
            canvas.height = canvas?.clientHeight;
        }

        // gl needs to convert from clip space values from gl_Position back to pixels
        gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);

        gl.clearColor(0.2, 0.2, 0.2, 1);
        gl.clear(gl.COLOR_BUFFER_BIT);

        gl.useProgram(program);

        // uniform values go after useProgram
        gl.uniform4f(colorLocation, 0.5, 0.2, 0.8, 1)
        gl.uniform2f(resolutionUniformLocation, gl.canvas.width, gl.canvas.height);

        // tell program which buffers to use and how to pull data from them
        gl.bindVertexArray(vao);

        // since we're drawing two triangles, count = 6
        gl.drawArrays(gl.TRIANGLES, 0, 6);
    });
</script>

<h1>Hello World</h1>

<canvas />
