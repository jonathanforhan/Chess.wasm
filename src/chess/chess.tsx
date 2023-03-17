import init, { moves, move_piece, validate, best_move } from "../../wasm/pkg/chess_wasm.js"

interface Move {
  to: String,
  from: String,
  promotion: String
}

interface MoveQueue {
  action: Move,
  reaction: Move,
};

class Chess {
  private _fen: string;
  private _stack: string[];
  private _moves: MoveQueue[];
  private _last_move: Move;

  constructor(fen: string) {
    this._fen = fen;
    if(this._fen === undefined || this._fen === "start") {
      this._fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    }
    this._stack = [];
    this._moves = [];
    this._last_move = undefined;
    init();
  }

  get fen() {
    return this._fen;
  }

  copy() {
    let chess = new Chess(this._fen);
    chess._stack = this._stack;
    chess._moves = this._moves;
    chess._last_move = this._last_move;
    return chess;
  }

  undo() {
    if(this._stack.length === 0) { return; }
    this._stack.pop();
    this._fen = this._stack.at(-1);
  }

  reset() {
    this._stack = [];
    this._fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
  }

  validate(fen: string) {
    try {
      validate(fen);
    } catch(e) {
      throw e;
    }
  }

  best_move(): Move {
    try {
      let result: Move;
      result = best_move(this._fen) as Move;
      // try to eliminate repeating moves
      let stack_copy = [
        this._stack[this._stack.length - 1].split(" ")[0],
        this._stack[this._stack.length - 5].split(" ")[0],
        this._stack[this._stack.length - 9].split(" ")[0],
      ];
      if(stack_copy[0] !== stack_copy[1] ||
        stack_copy[1] !== stack_copy[2] ||
        stack_copy[2] === undefined) {
        return result;
      } else {
        let result = moves(this._fen) as Move[];
        let i = Math.floor(Math.random() * moves.length);
        return result[i];
      }
    } catch(e) {
      console.log(e);
      if(e == "Engine Error: Checkmate") {
        alert(e);
      }
      else if(e == "Draw") {
        alert(e);
      } else {
        let result = moves(this._fen) as Move[];
        let i = Math.floor(Math.random() * moves.length);
        return result[i];
      }
    }
  }

  moves(): Move[] {
    const mvs: Move[] = moves(this._fen);
    return mvs;
  }

  move(mv: Move) {
    try {
      this._fen = move_piece(this._fen, mv);
      this._stack.push(this._fen);
      this._last_move = mv;
    } catch(e) {
      throw e;
    }
  }
}

export default Chess;
