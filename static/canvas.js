export function draw(x) {
    
    const canvas = document.getElementById("canvas")
    canvas.width = window.innerWidth
    canvas.height = window.innerHeight

    const ctx = canvas.getContext("2d")
    const circle = new Path2D()
    // console.log(x, "????????")
    circle.arc(x, 35, 10, 0, 2 * Math.PI);
    ctx.fillStyle = 'green';
    ctx.fill(circle);
    
    window.requestAnimationFrame(draw)
}
