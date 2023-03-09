import init, { moves, move_piece, validate } from "../../wasm/pkg/chess_wasm.js"

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
