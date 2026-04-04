import { Component, ElementRef, ViewChild, OnDestroy, AfterViewInit } from '@angular/core';

@Component({
  selector: 'app-matrix-background',
  standalone: true,
  template: `<canvas #matrixCanvas></canvas>`,
  styleUrls: ['./matrix-background.component.scss']
})
export class MatrixBackgroundComponent implements AfterViewInit, OnDestroy {
  @ViewChild('matrixCanvas') canvasRef!: ElementRef<HTMLCanvasElement>;
  private ctx!: CanvasRenderingContext2D;
  private animationId!: number;

  ngAfterViewInit() {
    this.initMatrix();
  }

  private initMatrix() {
    const canvas = this.canvasRef.nativeElement;
    this.ctx = canvas.getContext('2d')!;

    const characters = String.raw`ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$+-*/=%\"'#&_(),.;:?!\\|{}<>[]^~`;
    const fontSize = 16;
    let columns: number;
    let drops: number[] = [];

    const resize = () => {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;

      // Recalculamos columnas según el nuevo ancho
      columns = Math.floor(canvas.width / fontSize);

      // Preservamos las gotas actuales y añadimos nuevas si el ancho creció
      const oldLength = drops.length;
      if (columns > oldLength) {
        for (let i = oldLength; i < columns; i++) {
          drops[i] = Math.random() * -100; // Aparecen desde arriba con desfase
        }
      } else {
        drops.length = columns; // Recortamos si la ventana se achica
      }
    };

    // Escuchar el evento de la ventana
    window.addEventListener('resize', resize);
    resize(); // Ejecución inicial

    const draw = () => {
      this.ctx.fillStyle = "rgba(0, 0, 0, 0.05)";
      this.ctx.fillRect(0, 0, canvas.width, canvas.height);

      this.ctx.fillStyle = "#ff3e3e";
      this.ctx.font = fontSize + "px monospace";

      for (let i = 0; i < drops.length; i++) {
        const text = characters.charAt(Math.floor(Math.random() * characters.length));
        this.ctx.fillText(text, i * fontSize, drops[i] * fontSize);

        if (drops[i] * fontSize > canvas.height && Math.random() > 0.975) {
          drops[i] = 0;
        }
        drops[i]++;
      }
      this.animationId = requestAnimationFrame(draw);
    };

    draw();
  }
  ngOnDestroy() {
    cancelAnimationFrame(this.animationId);
  }
}