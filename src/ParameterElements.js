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
    const default_value = parameter.default.IntParam;
    const minimum = parameter.restriction.IntRangeRestriction.lower;
    const maximum = parameter.restriction.IntRangeRestriction.upper;
    const index = parameter.id;

    let value = formValues[index];

    const handleSliderChange = (event, newValue) => {
        setFormValues({
            ...formValues,
            [index]: newValue
        });
    };

    const handleInputChange = (event) => {
        setFormValues({
            ...formValues,
            [index]: (event.target.value === "" ? "" : Number(event.target.value))
        });
        onStlChange();
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
                    onChangeCommitted={onStlChange}
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
    const default_value = parameter.default.FloatParam;
    const minimum = parameter.restriction.FloatRangeRestriction.lower;
    const maximum = parameter.restriction.FloatRangeRestriction.upper;
    const index = parameter.id;

    let value = formValues[index];

    const handleSliderChange = (event, newValue) => {
        setFormValues({
            ...formValues,
            [index]: newValue
        });
    };

    const handleInputChange = (event) => {
        setFormValues({
            ...formValues,
            [index]: (event.target.value === "" ? "" : Number(event.target.value))
        });
        onStlChange();
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
                    onChangeCommitted={onStlChange}
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
    const maximum = parameter.restriction.StringLengthRestriction;
    const index = parameter.id;

    let value = formValues[index];

    const handleInputChange = (event) => {
        if (event.target.value.length <= maximum) {
            setFormValues({
                ...formValues,
                [index]: (event.target.value)
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
                <TextField
                    value={value}
                    onChange={handleInputChange}
                    onChangeCapture={onStlChange}
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
        onStlChange();
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
                    value={value}
                    size="small"
                    onChange={handleChange}
                    displayEmpty
                >
                    <MenuItem value={default_value}>{default_value}</MenuItem>
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
    const default_value = parameter.default.IntParam;
    const allowed = parameter.restriction.IntListRestriction;
    const index = parameter.id;

    return (ListRestriction(name, default_value, allowed, index, formValues, setFormValues, onStlChange))
}

// A ListRestriction for float parameters
export function FloatList(parameter, formValues, setFormValues, onStlChange) {
    const name = parameter.name;
    const default_value = parameter.default.FloatParam;
    const allowed = parameter.restriction.FloatListRestriction;
    const index = parameter.id;

    return (ListRestriction(name, default_value, allowed, index, formValues, setFormValues, onStlChange))
}

// A ListRestriction for string parameters
export function StringList(parameter, formValues, setFormValues, onStlChange) {
    const name = parameter.name;
    const default_value = parameter.default.StringParam;
    const allowed = parameter.restriction.StringListRestriction;
    const index = parameter.id;

    return (ListRestriction(name, default_value, allowed, index, formValues, setFormValues, onStlChange))
}

// A checkbox that represents the BoolRestriction
export function BoolCheck(parameter, formValues, setFormValues, onStlChange) {
    const name = parameter.name;
    const default_value = parameter.default.BoolParam;
    const index = parameter.id;

    const handleCheckChange = (event) => {
        setFormValues({
            ...formValues,
            [index]: (event.target.checked)
        });
        onStlChange();
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
    if (parameter.restriction.IntRangeRestriction) {
        return IntRange(parameter, formValues, setFormValues, onStlChange)
    } else if (parameter.restriction.IntListRestriction) {
        return IntList(parameter, formValues, setFormValues, onStlChange)
    } else if (parameter.restriction.FloatRangeRestriction) {
        return FloatRange(parameter, formValues, setFormValues, onStlChange)
    } else if (parameter.restriction.FloatListRestriction) {
        return FloatList(parameter, formValues, setFormValues, onStlChange)
    } else if (parameter.restriction.StringLengthRestriction) {
        return StringLength(parameter, formValues, setFormValues, onStlChange)
    } else if (parameter.restriction.StringListRestriction) {
        return StringList(parameter, formValues, setFormValues, onStlChange)
    } else if (parameter.restriction.NoRestriction) {
        return BoolCheck(parameter, formValues, setFormValues, onStlChange)
    }
}
