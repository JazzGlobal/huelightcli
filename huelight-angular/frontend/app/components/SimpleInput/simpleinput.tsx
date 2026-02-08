import { Input } from "../ui/input"
import { Button } from "../ui/button"
import { Field, FieldLabel } from "../ui/field"

function SimpleInput({
    label,
    value,
    onChange,
    placeholder,
}: {    
    label: string,
    value: string,
    onChange: (value: string) => void,
    placeholder?: string
}) {
    return (
        <Field className="m-1">
            <FieldLabel>{label}</FieldLabel>
            <Input value={value} onChange={e => onChange(e.target.value)} placeholder={placeholder}></Input>
        </Field>
    )
}

function SimpleInputButton({
    label,
    value,
    onChange,
    placeholder,
}: {    
    label: string,
    value: string,
    onChange: (value: string) => void,
    placeholder?: string
}) {
    return (
        <Field className="m-1">
            <FieldLabel>{label}</FieldLabel>
            <Button onClick={() => onChange(value)}>{placeholder}</Button>
        </Field>
    )
}

export {
    SimpleInput,
    SimpleInputButton
}