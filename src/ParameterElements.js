import React from "react";
import {
    Slider,
    Grid,
    Input,
    Typography,
    Select,
    MenuItem,
    TextField,
    Checkbox
} from "@mui/material";

// A slider, input combination that represents the IntRangeRestriction
export function IntRange(parameter, formValues, setFormValues, onStlChange) {
    const name = parameter.name;
    const default_value = parameter.default_value;
    const minimum = parameter.lower;
    const maximum = parameter.upper;
    const index = parameter.parameter_id;

    let value = formValues[index];

    const handleSliderChange = (event, newValue) => {
        setFormValues({
            ...formValues,
            [index]: newValue
        });
    };

    let prevValue;
    const handleSliderCommitted = (event, newValue) => {
        if (prevValue !== newValue) {
            onStlChange(index, newValue);
            prevValue = newValue;
        }
    }

    const handleInputChange = (event) => {
        setFormValues({
            ...formValues,
            [index]: (event.target.value === "" ? "" : Number(event.target.value))
        });
        onStlChange(index, (event.target.value === "" ? "" : Number(event.target.value)));
    };

    const handleBlur = () => {
        if (typeof value === "number") {
            if (value < minimum) {
                setFormValues({
                    ...formValues,
                    [index]: (minimum)
                });
            } else if (value > maximum) {
                setFormValues({
                    ...formValues,
                    [index]: (maximum)
                });
            } else {
                setFormValues({
                    ...formValues,
                    [index]: (Math.ceil(value))
                });
            }
        } else {
            setFormValues({
                ...formValues,
                [index]: (default_value)
            });
        }
    };

    return (
        <Grid container spacing={2} alignItems="center" className="Parameter-grid">
            <Grid item>
                <Typography>
                    {name}
                </Typography>
            </Grid>
            <Grid item xs>
                <Slider
                    value={typeof value === 'number' ? value : 0}
                    step={1}
                    min={minimum}
                    max={maximum}
                    onChange={handleSliderChange}
                    onChangeCommitted={handleSliderCommitted}
                    aria-labelledby={"int-range-slider"}
                />
            </Grid>
            <Grid item>
                <Input
                    value={value}
                    size="small"
                    onChange={handleInputChange}
                    onBlur={handleBlur}
                    inputProps={{
                        // FIXME: Dynamically change the step, maybe `floor(max/10)`
                        step: 1,
                        min: minimum,
                        max: maximum,
                        type: 'number',
                        'aria-labelledby': 'int-range-slider',
                    }}
                    style={{width: 60}}

               />
            </Grid>
        </Grid>
    );
}

// A slider, input combination that represents the FloatRangeRestriction
export function FloatRange(parameter, formValues, setFormValues, onStlChange) {
    const name = parameter.name;
    const default_value = parameter.default_value;
    const minimum = parameter.lower;
    const maximum = parameter.upper;
    const index = parameter.parameter_id;

    let value = formValues[index];

    const handleSliderChange = (event, newValue) => {
        setFormValues({
            ...formValues,
            [index]: newValue
        });
    };

    let prevValue;
    const handleSliderCommitted = (event, newValue) => {
        if (prevValue !== newValue) {
            onStlChange(index, newValue);
            prevValue = newValue;
        }
    }

    const handleInputChange = (event) => {
        setFormValues({
            ...formValues,
            [index]: (event.target.value === "" ? "" : Number(event.target.value))
        });
        onStlChange(index, (event.target.value === "" ? "" : Number(event.target.value)));
    };

    const handleBlur = () => {
        if (typeof value === "number") {
            if (value < minimum) {
                setFormValues({
                    ...formValues,
                    [index]: (minimum)
                });
            } else if (value > maximum) {
                setFormValues({
                    ...formValues,
                    [index]: (maximum)
                });
            }
        } else {
            setFormValues({
                ...formValues,
                [index]: (default_value)
            });
        }
    };

    return (
        <Grid container spacing={2} alignItems="center" className="Parameter-grid">
            <Grid item>
                <Typography>
                    {name}
                </Typography>
            </Grid>
            <Grid item xs>
                <Slider
                    value={typeof value === 'number' ? value : 0}
                    step={0.1}
                    min={minimum}
                    max={maximum}
                    onChange={handleSliderChange}
                    onChangeCommitted={handleSliderCommitted}
                    aria-labelledby={"float-range-slider"}
                />
            </Grid>
            <Grid item>
                <Input
                    value={value}
                    size="small"
                    onChange={handleInputChange}
                    onBlur={handleBlur}
                    inputProps={{
                        // FIXME: Dynamically change the step, maybe `floor(max/10)`
                        step: 0.05,
                        min: minimum,
                        max: maximum,
                        type: 'number',
                        'aria-labelledby': 'float-range-slider',
                    }}
                    style={{width: 60}}
                />
            </Grid>
        </Grid>
    );
}

// A TextField used to represent the StringLengthRestriction
export function StringLength(parameter, formValues, setFormValues, onStlChange) {
    const name = parameter.name;
    const length = parameter.length;
    const index = parameter.parameter_id;

    let value = formValues[index];

    const handleInputChange = (event) => {
        if (event.target.value.length <= length) {
            setFormValues({
                ...formValues,
                [index]: (event.target.value)
            });
        }
    };

    let prevValue;
    const handleInputCommitted = (event) => {
        if (prevValue !== event.target.value) {
            onStlChange(index, event.target.value);
            prevValue = event.target.value
        }
    }

    return (
        <Grid container spacing={2} alignItems="center" className="Parameter-grid">
            <Grid item>
                <Typography>
                    {name}
                </Typography>
            </Grid>
            <Grid item xs>
                <TextField
                    value={value}
                    onChange={handleInputChange}
                    onChangeCapture={handleInputCommitted}
                    size="small"
                />
            </Grid>
        </Grid>
    );
}

// A selection that represents a list restriction
function ListRestriction(name, default_value, allowed, index, formValues, setFormValues, onStlChange) {
    let value = formValues[index];

    const handleChange = (event) => {
        setFormValues({
            ...formValues,
            [index]: (event.target.value)
        });
        onStlChange(index, event.target.value);
    };

    return (
        <Grid container spacing={2} alignItems="center" className="Parameter-grid">
            <Grid item>
                <Typography>
                    {name}
                </Typography>
            </Grid>
            <Grid item>
                <Select
                    defaultValue={default_value}
                    value={value}
                    size="small"
                    onChange={handleChange}
                    displayEmpty
                >
                    {allowed.map(element => (
                        <MenuItem value={element}>{element}</MenuItem>
                    ))}
                </Select>
            </Grid>
        </Grid>
    );
}

// A ListRestriction for integer parameters
export function IntList(parameter, formValues, setFormValues, onStlChange) {
    const name = parameter.name;
    const default_value = parameter.default_value;
    const allowed = parameter.items;
    const index = parameter.parameter_id;

    return (ListRestriction(name, default_value, allowed, index, formValues, setFormValues, onStlChange))
}

// A ListRestriction for float parameters
export function FloatList(parameter, formValues, setFormValues, onStlChange) {
    const name = parameter.name;
    const default_value = parameter.default_value;
    const allowed = parameter.items;
    const index = parameter.parameter_id;

    return (ListRestriction(name, default_value, allowed, index, formValues, setFormValues, onStlChange))
}

// A ListRestriction for string parameters
export function StringList(parameter, formValues, setFormValues, onStlChange) {
    const name = parameter.name;
    const default_value = parameter.default_value;
    const allowed = parameter.items;
    const index = parameter.parameter_id;

    return (ListRestriction(name, default_value, allowed, index, formValues, setFormValues, onStlChange))
}

// A checkbox that represents the BoolRestriction
export function BoolCheck(parameter, formValues, setFormValues, onStlChange) {
    const name = parameter.name;
    const default_value = parameter.default_value;
    const index = parameter.parameter_id;

    const handleCheckChange = (event) => {
        setFormValues({
            ...formValues,
            [index]: (event.target.checked)
        });
        onStlChange(index, event.target.checked);
    };

    return (
        <Grid container spacing={2} alignItems="center" className="Parameter-grid">
            <Grid item>
                <Typography>
                    {name}
                </Typography>
            </Grid>
            <Grid item xs>
                <Checkbox
                    checked={default_value}
                    onChange={handleCheckChange}
                />
            </Grid>
        </Grid>
    );
}

export function RenderParam(parameter, formValues, setFormValues, onStlChange) {
    if (parameter.IntRange) {
        return IntRange(parameter.IntRange, formValues, setFormValues, onStlChange)
    } else if (parameter.IntList) {
        return IntList(parameter.IntList, formValues, setFormValues, onStlChange)
    } else if (parameter.FloatRange) {
        return FloatRange(parameter.FloatRange, formValues, setFormValues, onStlChange)
    } else if (parameter.FloatList) {
        return FloatList(parameter.FloatList, formValues, setFormValues, onStlChange)
    } else if (parameter.StringLength) {
        return StringLength(parameter.StringLength, formValues, setFormValues, onStlChange)
    } else if (parameter.StringList) {
        return StringList(parameter.StringList, formValues, setFormValues, onStlChange)
    } else if (parameter.Bool) {
        return BoolCheck(parameter.Bool, formValues, setFormValues, onStlChange)
    }
}
