interface Foo {
	fieldOne: number;
	fieldTwo: boolean;
	fieldThree: string;
	fieldFour: number[];
}

type ExternallyTagged =
	| {
 firstVariant: string; }

	| {
 secondVariant: number[]; }
;

type InternallyTagged =
	| {
 type: "firstVariant"; }
 & Foo
	| {
 type: "secondVariant"; }
 & {
 name: string; value: number; }
;

type AdjacentlyTagged =
	| {
 type: "firstVariant"; value: string; }

	| {
 type: "secondVariant"; value: {
 values: number[]; }
; }
;

type NewType = number;

type Generic<T> = T;

