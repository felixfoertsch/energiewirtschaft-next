declare module "edifact-json-transformer" {
	export function createTransformer(): {
		transform(input: string): Record<string, unknown>;
	};
}
