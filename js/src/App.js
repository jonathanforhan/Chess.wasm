import './App.css';
import { useState, useRef, useEffect } from 'react';
import { Chessboard } from 'react-chessboard';
import SideMenu from './components/SideMenu';
import useWindowDimensions from './util/useWindowDimensions';
import init, { Chess } from "./engine/chess_wasm.js"

function App() {
  const _window = useWindowDimensions();

  //const [game, setGame] = useState(new Chess());
  const [currentTimeout, setCurrentTimeout] = useState();
  const chessboardRef = useRef(null);

  useEffect(() => {
    init().then(() => {
      let chess = new Chess('4k2r/6r1/8/8/8/8/3R4/R3K3 w Qk - 0 1');
      console.log(chess.fen());
    })
  }, [])

  /**
   * @param {function} modify - modify function for the game Object
   * @returns {Object} update - updated game Object
   */
  //function safeGameMutate(modify) {
    //setGame((g) => {
      //const update = new Chess(g.get_fen());
      //modify(update);
      //return update;
    //});
  //}

  /**
   * @param {string} fen - fen notation of current position
   */
  function oppTurn(fen) {
    init().then(() => {
      let chess = new Chess(fen);
      console.log(chess.get_fen());
      //safeGameMutate(g => g.move({ from: "a7", to: "a5" }));
    });
  }

  /**
   * @param {string, string} src, dst - src square and destination dquare
   * @returns {boolean} - validity of move
   */
  //function onDrop(src, dst) {
    //let gameCopy = new Chess(game.fen());
    //try {
      //gameCopy.move({
        //from: src,
        //to: dst,
        //promotion: "q",
      //});
    //} catch {
      //return false;
    //}
    //setGame(gameCopy);
    //const newTimeout = setTimeout(oppTurn(gameCopy.fen()), 300);
    //setCurrentTimeout(newTimeout);
//
    //return true;
  //}

  return (
    <div className='App flex justify-center bg-slate-700 py-20 h-[100vh]'>
      <div className='ChessboardWrapper'>
        <Chessboard
          id='chessBoard'
          position={'start'}
          arePremovesAllowed={true}
          boardWidth={_window.height - 160}
          customBoardStyle={{
            borderRadius: "4px",
            boxShadow: "0 2px 10px rgba(0, 0, 0, 0.5)",
          }}
          customDarkSquareStyle={{ backgroundColor: "#779952" }}
          customLightSquareStyle={{ backgroundColor: "#edeed1" }}
          
        />
      </div>
      <SideMenu
        chessboardRef={chessboardRef}
        currentTimeout={currentTimeout}
      />
      </div>
  );
}

export default App;
