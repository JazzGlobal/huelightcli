import axios from "axios";

export interface LightDto {
    id: number;
    name: string;
    state: LightStateDto;
    type: string;
}

export interface LightStateDto {
    on: boolean;
    bri: number;
    hue: number;
    sat: number;
}

const api = axios.create({
    baseURL: "http://localhost:3000/api", // TODO: Read from config
});

export const getLights = async () => {
    try {
        const response = await api.get<LightDto[]>("/lights");
        return response.data;
    } catch (error) {
        console.error("Error fetching lights:", error);
        throw error;
    }
};