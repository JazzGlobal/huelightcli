import { inject, Injectable } from "@angular/core";
import { HttpClient } from "@angular/common/http";
import { Observable } from "rxjs";

export interface LightDto {
    id: number;
    name: string;
    state: LightState;
    type: string;
}

export interface LightState {
    on: boolean;
    bri: number;
    hue: number;
    sat: number;
}

@Injectable({ providedIn: 'root' })
export class LightService {
    private readonly baseUrl = 'http://localhost:3000/'; // TODO: Read from config
    private readonly http = inject(HttpClient);

    private readonly getLightsUrl = `${this.baseUrl}api/lights`;
    
    getLights(): Observable<LightDto[]> {
        return this.http.get<LightDto[]>(this.getLightsUrl);
    }
}