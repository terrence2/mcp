#include <iostream>
#include <memory>

#include <tclap/CmdLine.h>

#include "Network.h"
#include "Kinect.h"

using namespace std;

static void run(const string &name)
{
    Network link(name);
    Kinect kinect(&link);

    cout << "Started" << endl;
    kinect.loop();
    cout << "Finished" << endl;
}

int main(int argc, char **argv)
{
    TCLAP::CmdLine cmd("Opens a kinect and publishes events.", ' ', "0.0.0");
    TCLAP::ValueArg<string> nameArg("n","name","Name of this sensor.",true,"","string");
    cmd.add(nameArg);
    cmd.parse(argc, argv);

    try {
        run(nameArg.getValue());
    } catch(KinectError e) {
        cerr << "KinctError- " << e.message() << endl;
        return 1;
    }
    return 0;
}
