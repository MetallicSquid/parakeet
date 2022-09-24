import React from "react";
import {Button, Container, ListItem, Pagination, Stack, Typography} from "@mui/material";

export function TimeSinceUpdate(props) {
    const updateHours = (props.updateTime.getHours().toString().length === 2 ? props.updateTime.getHours() : "0" + props.updateTime.getHours());
    const updateMinutes = (props.updateTime.getMinutes().toString().length === 2 ? props.updateTime.getMinutes() : "0" + props.updateTime.getMinutes());

    return (
        <Typography>Last updated at {updateHours}:{updateMinutes}. </Typography>
    )
}

export function ModelDimensions(props) {
    return (
        <Typography>X: {props.dimensions[0]}, Y: {props.dimensions[1]}, Z: {props.dimensions[2]} (mm)</Typography>
    )
}

export function PartPagination(props) {
    if (props.numberOfParts !== 1) {
        return (
            <Stack
                direction="row"
                alignItems="center"
                justifyContent="center"
                spacing={2}
                pt={1}
            >
                <Typography><b>Parts: </b></Typography>
                <Pagination count={props.numberOfParts} onChange={props.handleChange} color="primary" />
            </Stack>
        )
    }
}

export function ButtonDownload(props) {
    return (
       <Button variant="outlined" href={props.stl} download>
           Download STL
       </Button>
    )
}
