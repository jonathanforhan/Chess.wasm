import { useEffect, useState, useRef } from 'react'
import { Chessboard } from 'react-chessboard';
import init, { Chess } from '../wasm/pkg/chess_wasm.js'
import './App.css'
import SideBar from './components/side-bar.js';
import useWindowDimensions from './hooks/use-window-dimensions.js';

function App() {
  const _window = useWindowDimensions();

  const [loading, setLoading] = useState(true);
  const [game, setGame] = useState({} as Chess);
  const [currentTimeout, setCurrentTimeout] = useState(0);
  const chessboardRef = useRef();
  
  useEffect(() => {
    init().then(() => {
      setGame(new Chess("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
      setLoading(false);
    })
  }, []);

  function safeGameMutate(modify: Function) {
    const update = new Chess(game.fen());
    modify(update);
    setGame(update);
  }

  function oppTurn(fen: string) {
    const gameCopy: Chess = new Chess(fen)
    let moves = gameCopy.moves();
    let i = Math.floor(Math.random() * moves.length);
    console.log(moves[i])
    safeGameMutate((g: Chess) => g.movePiece(moves[i]))
  }

  function onDrop(src: String, dst: String) {
    const gameCopy: Chess = new Chess(game.fen());
    try {
      gameCopy.movePiece({
        from: src,
        to: dst,
      });
    } catch(e) {
      console.log(e)
      return false;
    }
    setGame(gameCopy);
    oppTurn(gameCopy.fen())

    return true;
  }

  if(loading === true) return <></>;
  return (
    <div className="App">
      <div className="Title"><h1>Chess.wasm Demo</h1></div>
      <div className="Chess">
        <div>
          <Chessboard
            id='chessBoard'
            position={game.fen()}
            onPieceDrop={onDrop}
            arePremovesAllowed={true}
            isDraggablePiece={({ piece }) => piece[0] === "w"}
            boardWidth={_window.height - (_window.height / 5)}
            customBoardStyle={{
              borderRadius: "4px",
              boxShadow: "0 2px 10px rgba(0, 0, 0, 0.5)",
            }}
            customDarkSquareStyle={{ backgroundColor: "#779952" }}
            customLightSquareStyle={{ backgroundColor: "#edeed1" }}
          />
        </div>
        <SideBar/>
      </div>
    </div>
  )
}

export default App
