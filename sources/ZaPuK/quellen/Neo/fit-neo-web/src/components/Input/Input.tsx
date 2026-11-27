// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import './input.css';

interface CheckboxProps {
  type: 'checkbox';
  label: string;
  name: string;
  value?: string;
  checked?: boolean;
  defaultChecked?: boolean;
  disabled?: boolean;
  onChange?: (checked: boolean) => void;
}

interface RadioProps {
  type: 'radio';
  label: string;
  name: string;
  value: string;
  checked?: boolean;
  defaultChecked?: boolean;
  disabled?: boolean;
  onChange?: (value: string) => void;
}

type InputProps = CheckboxProps | RadioProps;

export default function Input({
  type,
  label,
  name,
  value,
  checked,
  defaultChecked,
  disabled,
  onChange,
}: InputProps) {
  const id = value !== undefined ? `${name}-${value}` : name;

  function handleChange(event: React.ChangeEvent<HTMLInputElement>) {
    if (!onChange) return;
    if (type === 'checkbox') {
      (onChange as CheckboxProps['onChange'])?.(event.target.checked);
    } else {
      (onChange as RadioProps['onChange'])?.(event.target.value);
    }
  }

  return (
    <label
      className={`input input-${type}${disabled ? ' input-disabled' : ''}`}
      htmlFor={id}
    >
      <input
        className="input-control"
        type={type}
        id={id}
        name={name}
        value={value}
        checked={checked}
        defaultChecked={defaultChecked}
        disabled={disabled}
        onChange={handleChange}
      />
      <span className="input-indicator" aria-hidden="true" />
      <span className="input-label">{label}</span>
    </label>
  );
}
