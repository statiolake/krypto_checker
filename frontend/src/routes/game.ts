const MIN_NUMBER: number = 1;
const MAX_NUMBER: number = 10;
const NUM_CARDS: number = 5;

const random = (): number => Math.floor(Math.random() * (MAX_NUMBER - MIN_NUMBER + 1)) + MIN_NUMBER;

export const generateQuiz = (): { cards: number[]; target: number } => {
	return {
		cards: [...Array(NUM_CARDS)].map(random).sort((a, b) => a - b),
		target: random()
	};
};

type Formula =
	| { type: 'number'; value: number }
	| { type: 'parened'; expr: Formula }
	| { type: 'binop'; exprs: Array<{ expr: Formula; op?: '+' | '-' | '*' | '/' }> };

class Parser {
	#index: number;
	#callCount: number;
	constructor(private input: string) {
		trace('--- parsing start ---');
		this.#callCount = 0;
		this.#index = 0;
	}

	skipWhitespace(): void {
		while (this.peekChar() === ' ') {
			this.nextChar();
		}
	}

	isFinished(): boolean {
		return this.#index === this.input.length;
	}

	nextChar(): string | undefined {
		if (this.#callCount++ > 1000) {
			throw new Error("I'm stuck");
		}

		if (this.isFinished()) {
			return undefined;
		}

		trace('next:', this.input[this.#index]);

		return this.input[this.#index++];
	}

	peekChar(): string | undefined {
		if (this.#callCount++ > 1000) {
			throw new Error("I'm stuck");
		}

		if (this.isFinished()) {
			return undefined;
		}

		trace('peek:', this.input[this.#index]);

		return this.input[this.#index];
	}

	eatChar(expect: string): boolean {
		const actual = this.nextChar();
		return actual === expect;
	}

	parseAdditive(): Formula | undefined {
		trace('parsing additive');
		const elements: Array<{ expr: Formula; op?: '+' | '-' }> = [];

		while (!this.isFinished()) {
			this.skipWhitespace();
			const expr = this.parseMultiplicative();
			if (expr == null) break;

			this.skipWhitespace();
			const op = this.peekChar();
			if (op === '+' || op === '-') {
				this.eatChar(op);
				elements.push({ expr, op });
			} else {
				elements.push({ expr });
				break;
			}
		}

		if (elements.length === 0) return undefined;
		return returnInspect({ type: 'binop', exprs: elements });
	}

	parseMultiplicative(): Formula | undefined {
		trace('parsing multiplicative');
		const elements: Array<{ expr: Formula; op?: '*' | '/' }> = [];

		while (!this.isFinished()) {
			this.skipWhitespace();
			const expr = this.parseTerminal();
			if (expr == null) break;

			this.skipWhitespace();
			const op = this.peekChar();
			if (op === '*' || op === '/') {
				this.eatChar(op);
				elements.push({ expr, op });
			} else {
				elements.push({ expr });
				break;
			}
		}

		if (elements.length === 0) return undefined;
		return returnInspect({ type: 'binop', exprs: elements });
	}

	parseTerminal(): Formula | undefined {
		trace('parsing terminal');
		this.skipWhitespace();
		if (this.peekChar() === '(') {
			this.eatChar('(');
			const expr = this.parseAdditive();
			if (expr == null) return undefined;
			if (!this.eatChar(')')) return undefined;
			return { type: 'parened', expr };
		} else {
			const value = this.parseNumber();
			if (value == null) return undefined;

			return returnInspect({ type: 'number', value });
		}
	}

	parseNumber(): number | undefined {
		trace('parsing number');
		let value = 0;
		let found = false;

		while (!this.isFinished()) {
			const ch = this.peekChar()!;
			if (!/[0-9]/.test(ch)) break;
			found = true;
			this.eatChar(ch);

			value = value * 10 + Number(ch);
		}

		return returnInspect(found ? value : undefined);
	}
}

const trace = (...args: unknown[]): void => {
	// console.log(...args);
};

const returnInspect = <T>(value: T): T => {
	trace('returning', value);
	return value;
};

export const parseFormula = (input: string): Formula | undefined => {
	const parser = new Parser(input);
	return parser.parseAdditive();
};

export const computeFormula = (formula: Formula): number => {
	switch (formula.type) {
		case 'number':
			return formula.value;
		case 'parened':
			return computeFormula(formula.expr);
		case 'binop': {
			let result = computeFormula(formula.exprs[0].expr);
			let op = formula.exprs[0].op;
			for (let i = 1; i < formula.exprs.length; i++) {
				const { expr, op: nextOp } = formula.exprs[i];
				if (op === '+') {
					result += computeFormula(expr);
				} else if (op === '-') {
					result -= computeFormula(expr);
				} else if (op === '*') {
					result *= computeFormula(expr);
				} else if (op === '/') {
					result /= computeFormula(expr);
				}
				op = nextOp;
			}
			return result;
		}
	}
};
