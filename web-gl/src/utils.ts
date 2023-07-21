function logGLCall(functionName: string, args: unknown[]) {
    console.log(
        'gl.' +
            functionName +
            '(' +
            window.WebGLDebugUtils.glFunctionArgsToString(functionName, args) +
            ')'
    );
}

export function initializeWebGlContext() {
    const canvas = document.querySelector('canvas');
    const gl = window.WebGLDebugUtils.makeDebugContext(
        canvas?.getContext('webgl2'),
        undefined,
        logGLCall
    );

    if (!gl) {
        throw new Error('Failed to initialize WebGL context.');
    }

    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT);

    return gl;
}

export function loadShader(gl: WebGL2RenderingContext, type: number, source: string) {
    const shader = gl.createShader(type);

    if (!shader) {
        return;
    }

    gl.shaderSource(shader, source);
    gl.compileShader(shader);

    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        console.error('Failed to compile shaders: ' + gl.getShaderInfoLog(shader));
        gl.deleteShader(shader);
        return;
    }

    return shader;
}
