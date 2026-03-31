import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { IonApp, IonContent, IonHeader, IonToolbar, IonTitle, IonInput, IonButton, IonItem, IonLabel } from '@ionic/angular/standalone';
import { invoke } from '@tauri-apps/api/core';
import { FormsModule } from '@angular/forms'; // <--- 1. Importación necesaria

@Component({
  selector: "app-root",
  standalone: true,
  imports: [RouterOutlet, IonApp, IonContent, IonHeader, IonToolbar, IonTitle, IonInput, IonButton, IonItem, IonLabel, FormsModule],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
})
export class AppComponent {
  url: string = '';
  titulo: string = '';

  async getMetadata() {
    // Verificación de seguridad profesional
    if (!(window as any).__TAURI_INTERNALS__) {
      this.titulo = "Error: No se detecta el entorno de Tauri. ¿Estás en el navegador?";
      return;
    }

    try {
      this.titulo = await invoke('check_video_url', { url: this.url });
    } catch (error) {
      console.error("Error desde Rust:", error);
      this.titulo = "Error de Rust: " + error;
    }
  }
}
