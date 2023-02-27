import './App.css';
import { useState, useEffect } from 'react';
import { Chessboard } from 'react-chessboard';
import Chess from './chess/chess';
import SideBar from './components/side-bar.js';
import useWindowDimensions from './hooks/use-window-dimensions.js';
    
function App() {
  const _window = useWindowDimensions();

  const [game, setGame] = useState(new Chess(""));
  const [turn, setTurn] = useState(false);

  function oppTurn() {
    let gameCopy: Chess = new Chess(game.fen());
    let moves = gameCopy.moves();
    let i = Math.floor(Math.random() * moves.length);
    console.log(moves[i]);
    console.log(gameCopy.fen())
    gameCopy.move(moves[i]);
    setGame(gameCopy);
    setTurn(false);
  }

  useEffect(() => {
    if(!turn) return;
    oppTurn();
  }, [turn]);

  function onDrop(src: String, dst: String) {
    let gameCopy: Chess = new Chess(game.fen());
    try {
      gameCopy.move({
        from: src,
        to: dst,
      });
    } catch(e) {
      console.log(e)
      return false;
    }
    setGame(gameCopy)
    setTurn(true);
    return true;
  }

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
