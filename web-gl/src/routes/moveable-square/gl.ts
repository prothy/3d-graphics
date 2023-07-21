import { initializeWebGlContext } from '../../utils';

import vsSource from './shader.vert?raw';
import fsSource from './shader.frag?raw';

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//    https://webgl2fundamentals.org/webgl/lessons/webgl-fundamentals.html    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

export default class GL {
    gl: WebGL2RenderingContext;
    vertexShader: WebGLShader;
    fragmentShader: WebGLShader;

    program: WebGLProgram;

    positionAttributeLocation: number;
    resolutionUniformLocation: WebGLUniformLocation;
    colorLocation: WebGLUniformLocation;

    constructor() {
        this.gl = initializeWebGlContext();

        this.vertexShader = this.createShader(this.gl.VERTEX_SHADER, vsSource);
        this.fragmentShader = this.createShader(this.gl.FRAGMENT_SHADER, fsSource);

        this.program = this.createProgram();

        const positionAttributeLocation = this.gl.getAttribLocation(this.program, 'a_position');
        const resolutionUniformLocation = this.gl.getUniformLocation(this.program, 'u_resolution');
        const colorLocation = this.gl.getUniformLocation(this.program, 'u_color');

        if (!(resolutionUniformLocation && colorLocation)) {
            throw new Error('Failed to initialize locations');
        }

        this.positionAttributeLocation = positionAttributeLocation;
        this.resolutionUniformLocation = resolutionUniformLocation;
        this.colorLocation = colorLocation;
    }

    private createShader(type: number, source: string) {
        const shader = this.gl.createShader(type);

        if (!shader) {
            throw new Error('Failed to create shader');
        }

        this.gl.shaderSource(shader, source);
        this.gl.compileShader(shader);

        const success = this.gl.getShaderParameter(shader, this.gl.COMPILE_STATUS);

        if (success) {
            return shader;
        }

        console.log(this.gl.getShaderInfoLog(shader));
        this.gl.deleteShader(shader);

        throw new Error('Failed to create shader');
    }

    private createProgram() {
        const program = this.gl.createProgram();

        if (!program) {
            throw new Error('Failed to create program');
        }

        this.gl.attachShader(program, this.vertexShader);
        this.gl.attachShader(program, this.fragmentShader);

        this.gl.linkProgram(program);

        const success = this.gl.getProgramParameter(program, this.gl.LINK_STATUS);

        if (success) {
            return program;
        }

        console.log(this.gl.getProgramInfoLog(program));
        this.gl.deleteShader(program);

        throw new Error('Failed to create program');
    }

    handleBuffer(positionsArray: number[]) {
        const positionBuffer = this.gl.createBuffer();

        this.gl.bindBuffer(this.gl.ARRAY_BUFFER, positionBuffer);

        // new positions for rectangle
        const positions = positionsArray;

        this.gl.bufferData(this.gl.ARRAY_BUFFER, new Float32Array(positions), this.gl.STATIC_DRAW);

        const vao = this.gl.createVertexArray();
        this.gl.bindVertexArray(vao);

        this.gl.enableVertexAttribArray(this.positionAttributeLocation);

        const size = 2; // 2 components per iteration
        const type = this.gl.FLOAT; // data is 32-bit floags
        const normalize = false;
        const stride = 0;
        const offset = 0;

        this.gl.vertexAttribPointer(
            this.positionAttributeLocation,
            size,
            type,
            normalize,
            stride,
            offset
        );

        this.initializeCanvas();

        this.gl.bindVertexArray(vao);
    }

    private initializeCanvas() {
        const canvas = document.querySelector('canvas');

        if (canvas) {
            canvas.width = canvas?.clientWidth;
            canvas.height = canvas?.clientHeight;
        }

        // gl needs to convert from clip space values from gl_Position back to pixels
        this.gl.viewport(0, 0, this.gl.canvas.width, this.gl.canvas.height);

        this.gl.clearColor(0.2, 0.2, 0.2, 1);
        this.gl.clear(this.gl.COLOR_BUFFER_BIT);

        this.gl.useProgram(this.program);

        // uniform values go after useProgram
        this.gl.uniform4f(this.colorLocation, 0.5, 0.2, 0.8, 1);
        this.gl.uniform2f(
            this.resolutionUniformLocation,
            this.gl.canvas.width,
            this.gl.canvas.height
        );
    }

    draw() {
        this.gl.drawArrays(this.gl.TRIANGLES, 0, 6);
    }
}
