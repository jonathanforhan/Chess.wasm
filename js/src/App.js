import './App.css';
import { useState, useRef, useEffect } from 'react';
import { Chessboard } from 'react-chessboard';
import SideMenu from './components/SideMenu';
import useWindowDimensions from './util/useWindowDimensions';
import init, { Chess } from "./wasm/chess_wasm.js"

function App() {
  const _window = useWindowDimensions();

  const [loading, setLoading] = useState(true);
  const [game, setGame] = useState(null);
  const [currentTimeout, setCurrentTimeout] = useState();
  const chessboardRef = useRef(null);

  useEffect(() => {
    init().then(() => {
      setGame(new Chess("8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50"));
      setLoading(false);
    })
  }, []);

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

  if(loading) { return <></> } else {
  return (
    <div className='App flex justify-center bg-slate-700 py-20 h-[100vh]'>
      <div className='ChessboardWrapper'>
        <Chessboard
          id='chessBoard'
          position={game.fen()}
          onPieceDrop={(src, dst) => console.log(src, dst, game.moves())}
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

} // else

export default App;
