import React from "react";
import styled from "styled-components";
import { CockroachLabsLogo } from "./cockroachlabs-logo";
import { AtoLogo } from "./ato-logo";
import { TwitterLogo } from "./twitter-logo";

const Background = styled.div`
  display: block;
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  z-index: -1;
  background: linear-gradient(
    113.48deg,
    #190f33 24.1%,
    #0037a5 52.79%,
    #6933ff 75.42%
  );
`;

const Card = styled.div`
  width: 1280px;
  height: 1280px;
  overflow: hidden;
  font-family: "Fira Sans", "Helvetica Neue", Helvetica, Arial, sans-serif;
  position: relative;
`;

const Title = styled.div`
  font-style: normal;
  font-weight: bold;
  font-size: 72px;
  line-height: 86px;
  color: #fff;
  position: absolute;
  width: 786px;
  left: 56px;
  top: 54px;
  margin: 0;
`;

const UserImage = styled.img`
  width: 375px;
  height: 375px;
  border-radius: 100%;
  position: absolute;
  top: 450px;
  left: 452px;
`;

const Phrase = styled.div`
  font-style: normal;
  text-align: center;
  color: #fff;
  font-weight: bold;
  font-size: 96px;
  line-height: 115px;
  position: absolute;
  width: 1050px;
  height: 230px;
  left: 104px;
  top: 876px;
`;

const User = ({ image, username }) => {
  const phrases = ["Does this work", "Is this right", "Was this fast"];
  return (
    <div>
      <UserImage src={image} />
      <Phrase>
        {phrases[Math.floor(Math.random() * phrases.length)]}, {username}?
      </Phrase>
    </div>
  );
};

const StyledCockroachLabsLogo = styled(CockroachLabsLogo)`
  position: absolute;
  top: 1165px;
  left: 56px;
  width: 475px;
  height: 67px;
`;

const StyledAtoLogo = styled(AtoLogo)`
  position: absolute;
  width: 223px;
  height: 159px;
  left: 1001px;
  top: 60px;
`;

const StyledTwitterLogo = styled(TwitterLogo)`
  position: absolute;
  width: 70px;
  height: 70px;
  left: 868px;
  top: 1164px;
`;

const TwitterHandle = styled.div`
  position: absolute;
  width: 270px;
  height: 58px;
  left: 947px;
  top: 1170px;

  font-family: Fira Sans;
  font-style: normal;
  font-weight: bold;
  font-size: 48px;
  line-height: 58px;
  color: #ffffff;
`;

export const ShoutOut = ({ image, username }) => (
  <Card>
    <Background width={1280} height={1280} strokeWidth={8} seed={username} />
    <StyledAtoLogo />
    <Title>Shorten the time between 'make it work' and 'make it fast'</Title>
    <User image={image} username={username} />

    <StyledCockroachLabsLogo />
    <StyledTwitterLogo />
    <TwitterHandle>@itsaydrian</TwitterHandle>
  </Card>
);
