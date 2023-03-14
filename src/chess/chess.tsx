import init, { moves, move_piece, validate, best_move } from "../../wasm/pkg/chess_wasm.js"

interface Chess {
  _fen: string;
  _stack: string[];
};

class Chess {
  constructor(fen: string) {
    this._fen = fen;
    if(this._fen === undefined || this._fen === "start") {
      this._fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    }
    this._stack = [];
    init();
  }

  copy() {
    let chess = new Chess(this._fen);
    chess._stack = this._stack;
    return chess;
  }

  fen() {
    return this._fen;
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

  best_move(): any {
    try {
      const mv: any = best_move(this._fen);
      return mv;
    } catch(e) {
      console.log(e);
      if(e == "No moves") {
        alert("Game Over");
      }
      try {
        const mvs: any = moves(this._fen);
        if(mvs.length === 0) {
          alert("Game Over");
        } else {
          return mvs[0];
        }
      } catch(e) {
        console.log("best move and move failed", e);
      }
    }
  }

  moves(): any {
    const mvs: any = moves(this._fen);
    return mvs;
  }

  move(mv: Object) {
    try {
      this._fen = move_piece(this._fen, mv);
      this._stack.push(this._fen);
    } catch(e) {
      throw e;
    }
  }
}

export default Chess;

