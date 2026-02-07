import type { LightDto } from "~/services/api"

export function LightList({ lightData }: { lightData: LightDto[] }) {
    return (
        <div>
            <h1>Light List</h1>
            <p>Here you can manage your lights.</p>
            {
                lightData.map(light => (
                    <div key={light.id}>
                        <h2>{light.name}</h2>
                        <p>Type: {light.type}</p>
                        <p>State: {light.state.on ? "On" : "Off"}</p>
                    </div>
                ))
            }
        </div>
    )
}