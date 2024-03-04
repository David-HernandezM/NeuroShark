import { useRef, useEffect, useState } from 'react'

const useCanvas = (draw: any) => {
  const ref = useRef(null)
  const [INITIATED, setINITIATED] = useState(false);
  
  useEffect(() => {
    
    const canvas: any = ref.current

    if (!canvas) {
        console.log("CANVAS IS NOT INITIATED");
        return;
    }

    const context = canvas.getContext('2d')
    let count = 0;
    let animationId: any;
    
    const renderer = () => {
      count++
      draw(context, count)
      animationId = window.requestAnimationFrame(renderer)
    }

    function initCanvas() {
        context.clearRect(0, 0, 280,280);
      
        context.fillStyle = 'rgba(0,0,0,0.5)';
        context.fillRect(0,0,280,280);
      }

    if (!INITIATED) {
        initCanvas();
        setINITIATED(true);
    }

    renderer()
    
    return () => {
      window.cancelAnimationFrame(animationId)
    }
  }, [draw])
  
  return ref
}

export default useCanvas