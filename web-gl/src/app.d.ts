// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface Platform {}
	}

	interface Window {
        WebGLDebugUtils: {
            makeDebugContext: (
                ctx?: WebGL2RenderingContext | null,
                arg1?: unknown,
                arg2?: (...args) => void
            ) => WebGL2RenderingContext;
            glFunctionArgsToString: (arg0: string, ...args) => unknown;
        };
    }
}

export {};
