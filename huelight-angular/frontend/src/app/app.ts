import { Component, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { WidgetFrame } from '../components/widget-frame/widget-frame';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, WidgetFrame],
  templateUrl: './app.html',
  styleUrl: './app.scss'
})
export class App {
  protected readonly title = signal('frontend');
}
