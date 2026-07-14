declare module "window-size/utils.js" {
	interface WindowSize {
		width: number;
		height: number;
	}

	const utils: {
		get(): WindowSize | undefined;
	};

	export = utils;
}
