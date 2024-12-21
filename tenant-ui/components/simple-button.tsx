"use client";
import { Button } from "@chakra-ui/react";
import React from "react";

export default function SimpleButton() {
  function but() {
    alert("yippy");
  }
  return <Button onClick={but}>Go now fast</Button>;
}
