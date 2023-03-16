import './App.css';
import { useState, useEffect } from 'react';
import { Chessboard } from 'react-chessboard';
import Chess from './chess/chess';
import SideBar from './components/side-bar.js';
import useWindowDimensions from './hooks/use-window-dimensions.js';

function App() {
  const _window = useWindowDimensions();

  const [game, setGame] = useState(new Chess("start"));
  const [turn, setTurn] = useState(false);

  function oppTurn() {
    let gameCopy: Chess = game.copy();
    try {
      let best_move = gameCopy.best_move();
      console.log("Best move:", best_move);
      gameCopy.move(best_move);
      setGame(gameCopy);
      console.log(gameCopy.fen)
      if(gameCopy.moves().length === 0) {
        setTimeout(() => alert("Game Over"), 100);
      }
      setTurn(false);
    } catch(e) {
      console.log(e);
    }
  }

  useEffect(() => {
    while(!turn) {
      setTimeout(async function() {
          await game.queue_moves();
      }, 100);
      return;
    };
    setTimeout(() => oppTurn(), 50);
  }, [turn]);

  function onDrop(src: String, dst: String) {
    let gameCopy: Chess = game.copy();
    try {
      gameCopy.move({
        from: src,
        to: dst,
        promotion: ''
      });
    } catch(e) {
      console.log(e)
      return false;
    }
    setGame(gameCopy)
    setTurn(true);
    return true;
  }

  function undo() {
    let gameCopy: Chess = game.copy();
    gameCopy.undo();
    setGame(gameCopy);
  }

  function reset() {
    let gameCopy: Chess = game.copy();
    gameCopy.reset();
    setGame(gameCopy);
  }

  return (
    <div className="App">
      <div className="Title"><h1>Chess.wasm Demo</h1></div>
      <div className="Chess">
        <div>
          <Chessboard
            id='chessBoard'
            position={game.fen}
            boardOrientation={'white'}
            onPieceDrop={onDrop}
            arePremovesAllowed={true}
            isDraggablePiece={({ piece }) => piece[0] === "w"}
            boardWidth={
              _window.width > 1000 ?
              _window.height - (_window.height / 5) :
              _window.width * 0.8
            }
            customBoardStyle={{
              borderRadius: "4px",
              boxShadow: "0 2px 10px rgba(0, 0, 0, 0.5)",
            }}
            customDarkSquareStyle={{ backgroundColor: "#779952" }}
            customLightSquareStyle={{ backgroundColor: "#edeed1" }}
          />
        </div>
        <SideBar
          width={_window.width}
          undo={undo}
          reset={reset}
          setTurn={setTurn}
        />
      </div>
    </div>
  )
}

export default App
