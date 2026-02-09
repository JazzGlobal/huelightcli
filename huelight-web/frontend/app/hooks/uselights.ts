import { useEffect, useState } from "react";
import { getLights, type LightDto } from "../services/api";

export function useLights() {
    const [lights, setLights] = useState<LightDto[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        getLights()
            .then(setLights)
            .catch((error) => setError(error.message))
            .finally(() => setLoading(false));
    }, []);

    return { lights, loading, error };
}