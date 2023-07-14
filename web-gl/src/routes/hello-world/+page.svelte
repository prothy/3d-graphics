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

        // looking up attribute locations should happen  before render loop!
        const positionAttributeLocation = gl.getAttribLocation(program, 'a_position');

        // attributes get data from buffers
        const positionBuffer = gl.createBuffer();

        // webgl resources can be manipulated through bind points (kind of like global variables within webgl)
        gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

        const positions = [0, 0, 0, 0.5, 0.7, 0];

        // copies position to gpu. it uses positionBuffer because we used gl.ARRAY_BUFFER bind point above
        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

        const vao = gl.createVertexArray();
        gl.bindVertexArray(vao);

        // this step is needed as otherwise attribute will have constant value. "turns on" attribute
        gl.enableVertexAttribArray(positionAttributeLocation);

        const size = 2; // 2 components per iteration
        const type = gl.FLOAT; // data is 32-bit floags
        const normalize = false;
        const stride = 0;
        const offset = 0;

        gl.vertexAttribPointer(positionAttributeLocation, size, type, normalize, stride, offset);

        // regarding size = 2:
        // in shader we defined a_position using vec4, looks like this 
        // a_position = {x: 0, y: 0, z: 0, w:0 }
        // settings size = 2 means we only set x and y. z and w will default to 0 and 1 respectively.

        // gl needs to convert from clip space values from gl_Position back to pixels
        gl.viewport(0, 0, gl.canvas.width, gl.canvas.height)

        gl.clearColor(0, 0, 0, 0);
        gl.clear(gl.COLOR_BUFFER_BIT);

        gl.useProgram(program);

        // tell program which buffers to use and how to pull data from them
        gl.bindVertexArray(vao);

        gl.drawArrays(gl.TRIANGLES, 0, 3);

        // regarding count = 3:
        // this means the vertex shader will be executed 3 times
        // first time a_position.x and a_position.y will be set to the first two values in positions array
        // then next 2 and so on.
    });
</script>

<h1>Hello World</h1>

<canvas />
