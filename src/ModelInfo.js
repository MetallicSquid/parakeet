import React from "react";
import {Button, Typography} from "@mui/material";

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

export function ButtonDownload(props) {
    return (
       <Button variant="outlined" href={props.stl} download>
           Download STL
       </Button>
    )
}
