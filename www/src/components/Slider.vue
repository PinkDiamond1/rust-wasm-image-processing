<template>
    <div>
        <p>Hello from Slider</p>
        <canvas id="canvas" width="300" height="10"></canvas>
    </div>
</template>

<script>
    import init, { Slider, wasm_memory } from 'rust-slider';

    init().then(() => {
        const display = Slider.new(30);
        const pixel = new Uint8Array(wasm_memory().buffer, display.pixel(), 30)

        const canvas = $('#canvas')[0];
        const ctx = canvas.getContext('2d', { alpha: false });
        setInterval(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            for (let i = 0; i < pixel.length; i++) {
                if (pixel[i] > 0) {
                    // Intensity of RED depends on value in shared buffer
                    ctx.fillStyle = `hsl(0, 100%, ${100 - 10 * pixel[i]}%)`;
                    ctx.fillRect(i * 10, 0, 10, 10);
                }
            }

            display.next();
        }, 50);
    });
</script>

<style scoped>
a {
  color: #42b983;
}
</style>
