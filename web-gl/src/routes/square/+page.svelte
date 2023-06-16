<script lang="ts">
    import { onMount } from 'svelte';
    import { initializeWebGlContext, loadShader } from '../../utils';
    import { mat4 } from 'gl-matrix';
    import vsSource from './shader.vert?raw';
    import fsSource from './shader.frag?raw';

    function initShaderProgram(gl: WebGL2RenderingContext) {
        const vertexShader = loadShader(gl, gl.VERTEX_SHADER, vsSource);
        const fragmentShader = loadShader(gl, gl.FRAGMENT_SHADER, fsSource);

        const shaderProgram = gl.createProgram();

        if (!(shaderProgram && vertexShader && fragmentShader)) {
            return;
        }

        gl.attachShader(shaderProgram, vertexShader);
        gl.attachShader(shaderProgram, fragmentShader);
        gl.linkProgram(shaderProgram);

        return shaderProgram;
    }

    type ProgramInfo = {
        program: WebGLProgram;
        attribLocations: Record<string, number>;
        uniformLocations: Record<string, WebGLUniformLocation>;
    };

    function setPositionAttribute(
        gl: WebGL2RenderingContext,
        buffers: Record<string, WebGLBuffer | null>,
        programInfo: ProgramInfo
    ) {
        const numComponents = 2;
        const type = gl.FLOAT;
        const normalize = false;
        const stride = 0;
        const offset = 0;

        gl.bindBuffer(gl.ARRAY_BUFFER, buffers.position);
        gl.vertexAttribPointer(
            programInfo.attribLocations.vertexPosition,
            numComponents,
            type,
            normalize,
            stride,
            offset
        );
    }

    export function initPositionBuffer(gl: WebGL2RenderingContext) {
        const positionBuffer = gl.createBuffer();
        const positions = [1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0];

        gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

        return positionBuffer;
    }

    function initBuffers(gl: WebGL2RenderingContext) {
        const positionBuffer = initPositionBuffer(gl);

        return {
            position: positionBuffer!
        };
    }

    function drawScene(
        gl: WebGL2RenderingContext,
        programInfo: ProgramInfo,
        buffers: { position: WebGLBuffer }
    ) {
        gl.clearColor(0, 0, 0, 1);
        gl.clearDepth(1);
        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);

        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

        const fieldOfView = (45 * Math.PI) / 180;
        const aspect =
            (gl.canvas as HTMLCanvasElement).clientWidth /
            (gl.canvas as HTMLCanvasElement).clientHeight;
        const zNear = 0.1;
        const zFar = 100.0;

        // Projection Matrix: Defines where the camera is placed in the world
        // https://learnopengl.com/Getting-started/Coordinate-Systems
        const projectionMatrix = mat4.create();
        mat4.perspective(projectionMatrix, fieldOfView, aspect, zNear, zFar);

        // The Model-View Matrix: Model refers to the matrix that contains local space coordinates of individual models. View to the placement of these objects within the world space, from a top-down perspective (negative Z-axis)
        const modelViewMatrix = mat4.create();
        mat4.translate(modelViewMatrix, modelViewMatrix, [-0, 0, -6]);

        // Also there is clip space, at the end of the vertex shader run. Anything outside a -1.0 and 1.0 range is not visible and will clipped (this value can be converted from other coordinate systems defined in projection matrix)

        setPositionAttribute(gl, buffers, programInfo);
        gl.useProgram(programInfo.program);

        // Uniform matrix: Matrix that stays constant between renders
        gl.uniformMatrix4fv(programInfo.uniformLocations.projectionMatrix, false, projectionMatrix);
        gl.uniformMatrix4fv(programInfo.uniformLocations.modelViewMatrix, false, modelViewMatrix);

        {
            const offset = 0;
            const vertexCount = 4;
            gl.drawArrays(gl.TRIANGLE_STRIP, offset, vertexCount);
        }
    }

    onMount(() => {
        const gl = initializeWebGlContext();

        if (!gl) {
            return;
        }

        const shaderProgram = initShaderProgram(gl);
        const buffers = initBuffers(gl);

        if (!shaderProgram) {
            return;
        }

        const programInfo = {
            program: shaderProgram,
            attribLocations: {
                vertexPosition: gl.getAttribLocation(shaderProgram, 'aVertexPosition')
            },
            uniformLocations: {
                projectionMatrix: gl.getUniformLocation(shaderProgram, 'uProjectionMatrix')!,
                modelViewMatrix: gl.getUniformLocation(shaderProgram, 'uModelViewMatrix')!
            }
        };

        drawScene(gl, programInfo, buffers);
    });
</script>

<h1>Square</h1>

<canvas />
