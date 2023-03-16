import init, { moves, move_piece, validate, best_move, queue_moves } from "../../wasm/pkg/chess_wasm.js"

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

  async queue_moves() {
    const mvs = await queue_moves(this._fen);
    console.log("Queued moves: ", mvs)
    this._moves = mvs as MoveQueue[];
  }

  best_move(): Move {
    try {
      let result: Move;
      for(let m of this._moves) {
        if(m.action.to === this._last_move.to &&
          m.action.from === this._last_move.from) {
          result = m.reaction;
        }
      }
      if(result === undefined) {
        console.log("Lazy error");
        result = best_move(this._fen) as Move;
      }
      return result;
    } catch(e) {
      console.log(e);
      if(e == "Checkmate") {
        alert(e);
      }
      if(e == "Draw") {
        alert(e);
      }
      //try {
        //const mvs: Move[] = moves(this._fen);
        //if(mvs.length === 0) {
          //alert("Game Over");
        //} else {
          //return mvs[0];
        //}
      //} catch(e) {
        //console.log("best move and move failed", e);
      //}
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

  private __move(mv: Move) {
    try {
      this._fen =  move_piece(this._fen, mv);
    } catch(e) {
      throw e;
    }
  }
}

export default Chess;

