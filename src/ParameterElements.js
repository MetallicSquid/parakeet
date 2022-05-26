import React from "react";
import {
    Slider,
    Box,
    Grid,
    Input,
    Typography,
    Select,
    MenuItem,
    TextField,
    Checkbox
} from "@mui/material";

// A slider, input combination that represents the IntRangeRestriction
export function IntRange(parameter) {
    const name = parameter.name;
    const default_value = parameter.default.IntParam;
    const minimum = parameter.restriction.IntRangeRestriction.lower;
    const maximum = parameter.restriction.IntRangeRestriction.upper;

    const [value, setValue] = React.useState(default_value);

    const handleSliderChange = (event, newValue) => {
        setValue(newValue);
    };

    const handleInputChange = (event) => {
        setValue(event.target.value === "" ? "" : Number(event.target.value));
    };

    const handleBlur = () => {
        if (typeof value === "number") {
            if (value < minimum) {
                setValue(minimum);
            } else if (value > maximum) {
                setValue(maximum);
            } else {
                setValue(Math.ceil(value));
            }
        } else {
            setValue(default_value);
        }
    };

    return (
        <Box sx={{ width: 350 }}>
            <Grid container spacing={2} alignItems="center">
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
        </Box>
    );
}

// A slider, input combination that represents the FloatRangeRestriction
export function FloatRange(parameter) {
    const name = parameter.name;
    const default_value = parameter.default.FloatParam;
    const minimum = parameter.restriction.FloatRangeRestriction.lower;
    const maximum = parameter.restriction.FloatRangeRestriction.upper;

    const [value, setValue] = React.useState(default_value);

    const handleSliderChange = (event, newValue) => {
        setValue(newValue);
    };

    const handleInputChange = (event) => {
        setValue(event.target.value === "" ? "" : Number(event.target.value));
    };

    const handleBlur = () => {
        if (typeof value === "number") {
            if (value < minimum) {
                setValue(minimum);
            } else if (value > maximum) {
                setValue(maximum);
            }
        } else {
            setValue(default_value);
        }
    };

    return (
        <Box sx={{ width: 350 }}>
            <Grid container spacing={2} alignItems="center">
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
        </Box>
    );
}

// A TextField used to represent the StringLengthRestriction
export function StringLength(parameter) {
    const name = parameter.name;
    const default_value = parameter.default.StringParam;
    const maximum = parameter.restriction.StringLengthRestriction;

    const [value, setValue] = React.useState(default_value);

    const handleInputChange = (event) => {
        if (event.target.value.length <= maximum) {
            setValue(event.target.value);
        }
    };

    return (
        <Box sx={{ width: 350 }}>
            <Grid container spacing={2} alignItems="center">
                <Grid item>
                    <Typography>
                        {name}
                    </Typography>
                </Grid>
                <Grid item xs>
                    <TextField
                        value={value}
                        onChange={handleInputChange}
                        size="small"
                    />
                </Grid>
            </Grid>
        </Box>
    );
}

// A selection that represents a list restriction
function ListRestriction(name, default_value, allowed) {
    const [value, setValue] = React.useState(default_value);

    const handleChange = (event) => {
        setValue(event.target.value);
    }

    return (
        <Box sx={{ width: 350 }}>
            <Grid container spacing={2} alignItems="center">
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
        </Box>
    );
}

// A ListRestriction for integer parameters
export function IntList(parameter) {
    const name = parameter.name;
    const default_value = parameter.default.IntParam;
    const allowed = parameter.restriction.IntListRestriction;

    return (ListRestriction(name, default_value, allowed))
}

// A ListRestriction for float parameters
export function FloatList(parameter) {
    const name = parameter.name;
    const default_value = parameter.default.FloatParam;
    const allowed = parameter.restriction.FloatListRestriction;

    return (ListRestriction(name, default_value, allowed))
}

// A ListRestriction for string parameters
export function StringList(parameter) {
    const name = parameter.name;
    const default_value = parameter.default.StringParam;
    const allowed = parameter.restriction.StringListRestriction;

    return (ListRestriction(name, default_value, allowed))
}

// A checkbox that represents the BoolRestriction
export function BoolCheck(parameter) {
    const name = parameter.name;
    const default_value = parameter.default.BoolParam;

    const [value, setValue] = React.useState(default_value);

    const handleCheckChange = (event) => {
        setValue(event.target.checked)
    }

    return (
        <Box sx={{ width: 350 }}>
            <Grid container spacing={2} alignItems="center">
                <Grid item>
                    <Typography>
                        {name}
                    </Typography>
                </Grid>
                <Grid item xs>
                    <Checkbox
                        checked={value}
                        onChange={handleCheckChange}
                    />
                </Grid>
            </Grid>
        </Box>
    );
}
