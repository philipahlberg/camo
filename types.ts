interface Foo {
	field_one: number;
	field_two: boolean;
	field_three: string;
}

interface Bar {
	field_four: number;
	field_five: Foo;
}

type FooOrBar =
	| Foo
	| Bar
	| number
	| "Simple";

