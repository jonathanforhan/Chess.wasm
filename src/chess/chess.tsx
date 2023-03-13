import init, { moves, move_piece, validate, best_move } from "../../wasm/pkg/chess_wasm.js"

interface Chess {
  _fen: string;
};

class Chess {
  constructor(fen: string) {
    this._fen = fen;
    if(this._fen === undefined || this._fen === "start") {
      this._fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    }
    init();
  }

  copy() {
    return new Chess(this._fen);
  }

  fen() {
    return this._fen;
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
    } catch(e) {
      throw e;
    }
  }
}

export default Chess;

