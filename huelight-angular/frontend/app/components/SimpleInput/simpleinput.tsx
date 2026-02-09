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
        <Field className=""> // TODO: variable for className maybe?
            <FieldLabel>{label}</FieldLabel>
            <Input value={value} onChange={e => onChange(e.target.value)} placeholder={placeholder}></Input>
        </Field>
    )
}

function SimpleInputButton({
    label,
    onClick,
    value,
    labelContent,
}: {    
    label: string,
    value: string,
    onClick?: () => void,
    labelContent?: React.ReactNode
}) {
    return (
        <Field className="w-fit">
            <FieldLabel>{label} {labelContent}</FieldLabel>
            <Button onClick={onClick}>{value}</Button>
        </Field>
    )
}

export {
    SimpleInput,
    SimpleInputButton
}