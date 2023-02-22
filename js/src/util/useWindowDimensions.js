import { useState, useEffect } from 'react';

  /**
   * @returns {number, number} - width and height of browser window
   */
function getWindowDimensions() {
  const { innerWidth: width, innerHeight: height } = window;
  return { width, height };
}

  /**
   * @returns {Object} windowDimensions - Object windowDimensions { width, height }
   */
function useWindowDimensions() {
  const [windowDimensions, setWindowDimensions] = useState(getWindowDimensions());

  useEffect(() => {
    const handleResize = () => setWindowDimensions(getWindowDimensions());

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, [])

  return windowDimensions;
}

export default useWindowDimensions;
