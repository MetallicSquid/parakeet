import React from "react";
import {
    Slider,
    Box,
    Grid,
    Input,
    Typography,
    Select,
    MenuItem
} from "@mui/material";

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
                            step: 10,
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

export function IntList(parameter) {
    const name = parameter.name;
    const default_value = parameter.default.IntParam;
    const allowed = parameter.restriction.IntListRestriction;

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
    )
}
