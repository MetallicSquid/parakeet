import React from "react";
import {Tooltip, Typography} from "@mui/material";

export class TimeSinceUpdate extends React.Component {
    constructor(props) {
        super(props);
        const date_hours = (new Date(Date.now()).getHours()).toString();
        const date_minutes = (new Date(Date.now()).getMinutes()).toString();
        this.state = {
            start: Date.now(),
            hours: date_hours.length === 1 ? "0" + date_hours : date_hours,
            minutes: date_minutes.length === 1 ? "0" + date_minutes : date_minutes,
            date: new Date(Date.now()).toDateString(),
            duration_string: "0 seconds ago."
        }
    }

    render() {
        const calculateDurationString = () => {
            const time = Date.now() - this.state.start;
            const seconds = Math.floor(time / 1000);
            const minutes = Math.floor(time / 60000);
            const hours = Math.floor(time / 3600000);

            if (seconds < 60) {
                this.setState({ duration_string: (seconds + (seconds === 1 ? " second " : " seconds ") + "ago.") });
            } else if (minutes < 60) {
                this.setState({ duration_string: (minutes + (minutes === 1 ? " minute " : " minutes ") + "ago.") });
            } else if (hours < 24) {
                this.setState({ duration_string: (hours + (hours === 1 ? " hour " : " hours ") + "ago.") });
            } else {
                this.setState({ duration_string: "More than a day ago." });
            }
        }

        return (
            <Tooltip title={this.state.duration_string} onOpen={calculateDurationString}>
                <Typography>Last updated at {this.state.hours}:{this.state.minutes} on {this.state.date}.</Typography>
            </Tooltip>
        )
    }
}
